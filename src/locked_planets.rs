use crate::hoops_boops_loops::{LoopInfo, Planet, spawn_loop};
use crate::scales::ZOOMED_OUT_PLANET_SCALE;
use bevy::prelude::*;

#[derive(Component)]
pub struct LockedPlanet;

pub fn spawn_locked_planet(
    planet: Planet,
    pos: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    let img = asset_server.load(planet.get_locked_path());
    commands
        .spawn((
            Sprite::from_image(img),
            Transform {
                translation: pos.extend(0.),
                ..default()
            },
            LockedPlanet,
            Pickable::default(),
        ))
        .observe(spawn_loop_on_click)
        .id()
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
