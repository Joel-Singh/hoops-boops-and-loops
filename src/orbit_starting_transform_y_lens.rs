use crate::hoops_boops_loops::Orbit;
use bevy::prelude::*;
use bevy_tweening::*;

/// Lens used for zooming in and out of planets
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OrbitStartingTransformYLens {
    /// The start and end, default scale is 1
    /// Less than 1 is zoom in, more is zoom out
    pub start: f32,
    pub end: f32,
}

impl Lens<Orbit> for OrbitStartingTransformYLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Orbit>, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.starting_transform.translation.y = value;
    }
}

pub fn orbit_starting_transform_y_lens_plugin(app: &mut App) {
    app.add_systems(
        Update,
        component_animator_system::<Orbit>.in_set(AnimationSystem::AnimationUpdate),
    );
}
