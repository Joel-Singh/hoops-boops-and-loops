use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use std::time::Duration;

use super::STARTING_LEFT_POSITION;

pub fn slide_in_from_right_tween() -> Tween<Node> {
    return Tween::new(
        EaseFunction::BackOut,
        Duration::from_secs_f32(16.),
        UiPositionLens {
            start: UiRect::left(STARTING_LEFT_POSITION),
            end: UiRect::left(Val::Px(0.)),
        },
    );
}

pub fn wait_seconds(delay: f32) -> Tween<Node> {
    return Tween::new(
        EaseFunction::BackOut,
        Duration::from_secs_f32(delay),
        UiPositionLens {
            start: UiRect::left(STARTING_LEFT_POSITION),
            end: UiRect::left(STARTING_LEFT_POSITION),
        },
    );
}
