mod tweens;

use crate::hoops_boops_loops::{LoopInfo, Planet, spawn_loop};
use crate::locked_planets::tweens::slide_in_from_left_tween;
use crate::scales::ZOOMED_OUT_PLANET_SCALE;
use crate::screen_size::SCREEN_SIZE;
use bevy::prelude::*;
use bevy_tweening::Animator;

#[derive(Component)]
pub struct LockedPlanet {
    planet: Planet,
}

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
    pub planet: Planet,
    pub initial_scale: f32,
}

impl Command for SpawnLockedPlanet {
    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let planet_img = asset_server.load(self.planet.get_sprite_path());
        let loot_symbol = asset_server.load("loot-symbol.png");
        let spacey_font = asset_server.load("SpaceGrotesk-Light.ttf");

        let handles = world.get_resource::<Handles>().unwrap();
        let prehover_img = handles.prehover.clone();

        let price_display = spawn_price_display(
            &mut world.commands(),
            loot_symbol,
            spacey_font,
            self.pos.clone(),
            50,
        );

        let locked_planet = world
            .spawn((
                Sprite::from_image(prehover_img),
                Transform {
                    translation: self.pos.extend(0.),
                    scale: Vec3::splat(self.initial_scale),
                    ..default()
                },
                LockedPlanet {
                    planet: self.planet,
                },
                Pickable::default(),
            ))
            .observe(spawn_loop_on_click)
            .observe(highlight_on_hover)
            .observe(unhighlight_on_out)
            .observe(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
                commands.entity(price_display).despawn();
            })
            .id();

        world
            .entity_mut(locked_planet)
            .with_child(Sprite::from_image(planet_img));
    }
}

fn spawn_loop_on_click(
    t: Trigger<Pointer<Click>>,
    transform_q: Query<&Transform, With<LockedPlanet>>,
    locked_planet_q: Query<&LockedPlanet>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let position = transform_q.get(t.target).unwrap();
    let planet = locked_planet_q.get(t.target).unwrap().planet;

    let (r#loop, _, _) = spawn_loop(
        LoopInfo {
            position: position.translation.truncate(),
            planet: planet,
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

    commands.entity(t.target).despawn();
}

fn highlight_on_hover(t: Trigger<Pointer<Over>>, mut commands: Commands, handles: Res<Handles>) {
    commands
        .entity(t.target)
        .insert(Sprite::from_image(handles.onhover.clone()));
}

fn unhighlight_on_out(t: Trigger<Pointer<Out>>, mut commands: Commands, handles: Res<Handles>) {
    commands
        .entity(t.target)
        .insert(Sprite::from_image(handles.prehover.clone()));
}

fn spawn_price_display(
    commands: &mut Commands,
    loot_symbol: Handle<Image>,
    spacey_font: Handle<Font>,
    pos: Vec2,
    price: i32,
) -> Entity {
    // For animation
    const STARTING_LEFT_OFFSET: f32 = -1500.;
    const TOP_OFFSET: f32 = 135.;
    let top = (SCREEN_SIZE.y / 2.) - pos.y + TOP_OFFSET;
    let width = 150.;

    let ending_left: f32 = (SCREEN_SIZE.x / 2.) + pos.x - width / 2.;

    let starting_left: f32 = STARTING_LEFT_OFFSET + ending_left;

    let price_display = commands
        .spawn((
            Node {
                height: Val::Px(50.),
                width: Val::Px(width),
                border: UiRect::all(Val::Px(1.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                top: Val::Px(top),
                left: Val::Px(starting_left),
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.1)),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
        ))
        .id();

    let loot_symbol = commands
        .spawn((
            ImageNode::new(loot_symbol),
            Node {
                width: Val::Px(30.),
                height: Val::Px(30.),
                margin: UiRect::left(Val::Px(10.)),
                ..default()
            },
        ))
        .id();

    let text_container = commands
        .spawn(Node {
            width: Val::Px(80.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .id();

    commands.entity(text_container).with_child((
        Text::new(price.to_string()),
        TextFont {
            font: spacey_font,
            font_size: 40.,
            ..default()
        },
        Node {
            margin: UiRect::left(Val::Px(-5.)),
            ..default()
        },
    ));

    commands
        .entity(price_display)
        .add_children(&[loot_symbol, text_container])
        .insert(Animator::new(slide_in_from_left_tween(
            starting_left,
            ending_left,
            top,
        )));

    price_display
}

fn load_handles(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(Handles {
        onhover: asset_server.load("locked-planet/onhover.png"),
        onhover_moon: asset_server.load("locked-planet/onhover_moon.png"),

        prehover: asset_server.load("locked-planet/prehover.png"),
        prehover_moon: asset_server.load("locked-planet/prehover_moon.png"),
    });
}
