use crate::transition_to_first_planet::TransitionToFirstPlanet;
use bevy::prelude::*;

/// Marker struct for the art of the Titlescreen
#[derive(Component)]
pub struct TitlescreenArt;

/// Marker struct for the art of the Titlescreen
#[derive(Component)]
pub struct TitlescreenBtn;

/// Marker struct for the initial moon on the titlescreen
#[derive(Component)]
pub struct TitlescreenMoon;

/// The parent of all titlescreen entities
#[derive(Component)]
pub struct TitlescreenParent;

/// In World coords
pub const PLAY_BTN_LOCATION: Vec2 = Vec2::new(-54., 77.);

/// Spawns the titlescreen and play btn
pub fn titlescreen_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_title_screen);
}

/// Spawns the Titlescreen, Which will call transition to first planet on play_btn click
fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let art_image = asset_server.load("titlescreen/art.png");
    let play_btn_image = asset_server.load("titlescreen/play-button.png");
    let initial_moon_image = asset_server.load("titlescreen/initial-moon.png");

    let parent = commands
        .spawn((TitlescreenParent, Transform { ..default() }))
        .id();

    let art = commands
        .spawn((
            TitlescreenArt,
            Sprite::from_image(art_image),
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
            Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
        ))
        .id();

    let btn = commands
        .spawn((
            TitlescreenBtn,
            Sprite {
                image: play_btn_image,
                color: Color::WHITE.with_alpha(0.),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
            Transform {
                translation: Vec3::new(0., 0., 1.),
                ..default()
            },
        ))
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.queue(TransitionToFirstPlanet);
        })
        .id();

    // Represents the initial moon that turns into the play button on hover
    let initial_moon = commands
        .spawn((
            Sprite {
                image: initial_moon_image,
                ..default()
            },
            Pickable::default(),
            Transform {
                translation: Vec3::new(0., 0., -1.),
                ..default()
            },
            TitlescreenMoon,
        ))
        .observe(move |_: Trigger<Pointer<Over>>, mut commands: Commands| {
            commands.entity(btn).entry::<Sprite>().and_modify(|mut s| {
                s.color = Color::WHITE;
            });
        })
        .observe(move |_: Trigger<Pointer<Out>>, mut commands: Commands| {
            commands.entity(btn).entry::<Sprite>().and_modify(|mut s| {
                s.color = Color::WHITE.with_alpha(0.);
            });
        })
        .id();

    commands
        .entity(parent)
        .add_children(&[art, btn, initial_moon]);
}
