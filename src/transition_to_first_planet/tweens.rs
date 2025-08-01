use crate::projection_scale_lens::ProjectionScaleLens;
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

pub fn center_to_first_planet() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(8.),
        TransformPositionLens {
            start: Vec3::default(),
            end: PLAY_BTN_LOCATION.extend(0.0),
        },
    )
}

pub fn zoom_camera_in() -> Tween<Projection> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(8.),
        ProjectionScaleLens {
            start: 1.,
            end: 1. / 4.,
        },
    )
}
