use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use std::time::Duration;

pub fn slide_in_from_left_tween(start: f32, end: f32, top: f32) -> Tween<Node> {
    return Tween::new(
        EaseFunction::BackOut,
        Duration::from_secs_f32(15.),
        UiPositionLens {
            start: UiRect::left(Val::Px(start)).with_top(Val::Px(top)),
            end: UiRect::left(Val::Px(end)).with_top(Val::Px(top)),
        },
    );
}
