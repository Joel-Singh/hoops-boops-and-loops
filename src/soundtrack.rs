use bevy::audio::PlaybackMode;
use bevy::prelude::*;

/// Plays the sound track
pub fn soundtrack_plugin(app: &mut App) {
    app.add_systems(
        Startup,
        |mut commands: Commands, asset_server: Res<AssetServer>| {
            commands.spawn((
                AudioPlayer::new(asset_server.load("soundtrack.ogg")),
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    ..default()
                },
            ));
        },
    );
}
