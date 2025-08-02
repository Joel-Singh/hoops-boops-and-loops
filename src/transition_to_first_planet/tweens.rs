use super::FIRST_PLANET_INITIAL_SCALE;
use crate::titlescreen::PLAY_BTN_LOCATION;
use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use std::time::Duration;

pub fn fade_to_transparent() -> Tween<Sprite> {
    Tween::new(
        EaseFunction::Linear,
        Duration::from_secs_f32(5.),
        SpriteColorLens {
            start: Color::WHITE,
            end: Color::WHITE.with_alpha(0.0),
        },
    )
}

pub fn center_first_planet() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(8.),
        TransformPositionLens {
            start: PLAY_BTN_LOCATION.extend(0.0),
            end: Vec3::splat(0.0),
        },
    )
}

pub fn scale_up() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(8.),
        TransformScaleLens {
            start: Vec3::splat(1.),
            end: Vec3::splat(3.),
        },
    )
}

pub fn scale_planet_up() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(8.),
        TransformScaleLens {
            start: Vec3::splat(FIRST_PLANET_INITIAL_SCALE),
            end: Vec3::splat(1.),
        },
    )
}
