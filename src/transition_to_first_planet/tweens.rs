use super::FIRST_PLANET_INITIAL_SCALE;
use crate::titlescreen::PLAY_BTN_LOCATION;
use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use std::time::Duration;

/// The time for the planet and titlescreen to center. Will be the same so it looks like the screen
/// is moving
const CENTERING_TIME: f32 = 4.;

pub fn fade_to_transparent() -> Tween<Sprite> {
    Tween::new(
        EaseFunction::Linear,
        Duration::from_secs_f32(2.),
        SpriteColorLens {
            start: Color::WHITE,
            end: Color::WHITE.with_alpha(0.0),
        },
    )
}

pub fn center_first_planet() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(CENTERING_TIME),
        TransformPositionLens {
            start: PLAY_BTN_LOCATION.extend(0.0),
            end: Vec3::splat(0.0),
        },
    )
}

pub fn move_titlescreen_with_planet() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(CENTERING_TIME),
        TransformPositionLens {
            start: Vec3::splat(0.0),
            end: PLAY_BTN_LOCATION.extend(0.0) * -1.,
        },
    )
}

pub fn scale_planet_up() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(CENTERING_TIME),
        TransformScaleLens {
            start: Vec3::splat(FIRST_PLANET_INITIAL_SCALE),
            end: Vec3::splat(1.),
        },
    )
}
