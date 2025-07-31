use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;

pub fn background_plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_star_bg, set_background_color));
}

fn spawn_star_bg(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Sprite::from_image(
        asset_server.load("light-trail-bg-1.png"),
    ));

    commands.spawn(Sprite::from_image(
        asset_server.load("light-trail-bg-2.png"),
    ));
}

fn set_background_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BLACK.into()
}
