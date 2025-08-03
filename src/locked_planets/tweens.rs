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

pub fn fade_in_blue() -> Tween<Sprite> {
    Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(0.5),
        SpriteColorLens {
            start: Color::WHITE,
            end: Srgba::rgb(0.333, 0.808, 0.929).into(),
        },
    )
    .with_repeat_count(RepeatCount::Finite(2))
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
}

pub fn fade_in_orange() -> Tween<Sprite> {
    Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(0.5),
        SpriteColorLens {
            start: Color::WHITE,
            end: Srgba::rgb(1.0, 0.604, 0.259).into(),
        },
    )
    .with_repeat_count(RepeatCount::Finite(2))
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
}
