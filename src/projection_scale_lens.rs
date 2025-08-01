use bevy::prelude::*;
use bevy_tweening::*;

/// Lens used for zooming in and out of planets
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ProjectionScaleLens {
    /// The start and end, default scale is 1
    /// Less than 1 is zoom in, more is zoom out
    pub start: f32,
    pub end: f32,
}

impl Lens<Projection> for ProjectionScaleLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Projection>, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        let projection = target.target_mut();
        match projection {
            Projection::Orthographic(perspective) => {
                perspective.scale = dbg!(value);
            }
            _ => {
                panic!("Transitioning for non-orthographic views not supported");
            }
        }
    }
}

pub fn projection_scale_lens_plugin(app: &mut App) {
    app.add_systems(
        Update,
        component_animator_system::<Projection>.in_set(AnimationSystem::AnimationUpdate),
    );
}
