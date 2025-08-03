use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Event)]
pub struct PlayBoopThroughHoop;

pub fn plugin(app: &mut App) {
    app.add_event::<PlayBoopThroughHoop>()
        .add_systems(FixedUpdate, play_hoop_through_boop_sounds);
}

fn play_hoop_through_boop_sounds(
    mut sound_evs: EventReader<PlayBoopThroughHoop>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut time_since_last_boop: Local<Duration>,
    mut is_ready_to_boop: Local<bool>,
    time: Res<Time<Real>>,
) {
    const BOOP_INTERVAL: Duration = Duration::from_millis(150);

    *time_since_last_boop += time.delta();

    *is_ready_to_boop = *is_ready_to_boop || sound_evs.read().count() > 0;

    if *time_since_last_boop > BOOP_INTERVAL && *is_ready_to_boop {
        let sound = asset_server.load("boop-going-through-hoop.ogg");

        commands.spawn((
            AudioPlayer::new(sound.clone()),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        ));

        *time_since_last_boop = Duration::ZERO;
        *is_ready_to_boop = false;
    }
}
