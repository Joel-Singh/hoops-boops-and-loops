use crate::hoops_boops_loops::{LoopInfo, Planet, spawn_loop};
use crate::scales::ZOOMED_OUT_PLANET_SCALE;
use bevy::prelude::*;

#[derive(Component)]
pub struct LockedPlanet;

#[derive(Resource)]
struct Handles {
    onhover: Handle<Image>,
    onhover_moon: Handle<Image>,

    prehover: Handle<Image>,
    prehover_moon: Handle<Image>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_handles);
}

/// Command wrapper around spawn_locked_planet
pub struct SpawnLockedPlanet {
    pub pos: Vec2,
    pub initial_scale: f32,
}

impl Command for SpawnLockedPlanet {
    fn apply(self, world: &mut World) {
        let handles = world.get_resource::<Handles>().unwrap();

        world
            .spawn((
                Sprite::from_image(handles.prehover.clone()),
                Transform {
                    translation: self.pos.extend(0.),
                    scale: Vec3::splat(self.initial_scale),
                    ..default()
                },
                LockedPlanet,
                Pickable::default(),
            ))
            .observe(spawn_loop_on_click);
    }
}

fn spawn_loop_on_click(
    t: Trigger<Pointer<Click>>,
    transform_q: Query<&Transform, With<LockedPlanet>>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let position = transform_q.get(t.target).unwrap();

    let (r#loop, _, _) = spawn_loop(
        LoopInfo {
            position: position.translation.truncate(),
            planet: Planet::One,
            boop_prices: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            hoop_prices: [0, 0, 0, 0, 0, 0, 0, 0],
        },
        &mut commands,
        &asset_server,
    );

    commands
        .entity(r#loop)
        .entry::<Transform>()
        .and_modify(|mut t| t.scale = Vec3::splat(ZOOMED_OUT_PLANET_SCALE));
}

fn load_handles(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(Handles {
        onhover: asset_server.load("locked-planet/onhover.png"),
        onhover_moon: asset_server.load("locked-planet/onhover_moon.png"),

        prehover: asset_server.load("locked-planet/prehover.png"),
        prehover_moon: asset_server.load("locked-planet/prehover_moon.png"),
    });
}
