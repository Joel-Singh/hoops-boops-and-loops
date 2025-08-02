
use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use std::time::Duration;

const ZOOMED_OUT_PLANET_SCALE: f32 = 0.5;

/// Assuming its starting at Vec3::ZERO
pub fn move_first_planet(end: Vec3) -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(7.),
        TransformPositionLens {
            start: Vec3::ZERO,
            end,
        },
    )
}

/// Should scale down to the size of yuvi's art
pub fn zoom_planet_out() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(5.),
        TransformScaleLens {
            start: Vec3::splat(1.),
            end: Vec3::splat(ZOOMED_OUT_PLANET_SCALE),
        },
    )
}

pub fn zoom_planet_in_from_zero() -> Tween<Transform> {
    Tween::new(
        EaseFunction::SmoothStep,
        Duration::from_secs_f32(5.),
        TransformScaleLens {
            start: Vec3::splat(0.),
            end: Vec3::splat(ZOOMED_OUT_PLANET_SCALE),
        },
    )
}
