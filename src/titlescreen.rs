use crate::transition_to_first_planet::TransitionToFirstPlanet;
use bevy::prelude::*;

/// Marker struct for the art of the Titlescreen
#[derive(Component)]
pub struct TitlescreenArt;

/// Marker struct for the art of the Titlescreen
#[derive(Component)]
pub struct TitlescreenBtn;

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
    commands.spawn((
        TitlescreenArt,
        Sprite::from_image(art_image),
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
        Transform {
            translation: Vec3::new(0., 0., -1.),
            ..default()
        },
    ));

    commands
        .spawn((
            TitlescreenBtn,
            Sprite::from_image(play_btn_image),
            Pickable::default(),
        ))
        .observe(|trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.queue(TransitionToFirstPlanet);
        });
}
