mod tweens;

use crate::hoops_boops_loops::{LoopInfo, Planet, spawn_loop};
use crate::prices::*;
use crate::titlescreen::*;
use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;
use bevy_tweening::{Animator, Tracks};

use tweens::*;

pub const FIRST_PLANET_INITIAL_SCALE: f32 = 0.35;

/// Spawns the first planet, and transitions to it using a transition in CameraTransitions. Also sets up transitioning to the main game.
pub struct TransitionToFirstPlanet;
impl Command for TransitionToFirstPlanet {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached(transition_to_first_planet);
    }
}

fn transition_to_first_planet(
    titlescreen_btn: Single<Entity, With<TitlescreenBtn>>,
    titlescreen_art: Single<Entity, With<TitlescreenArt>>,
    titlescreen_moon: Single<Entity, With<TitlescreenMoon>>,
    titlescreen_parent: Single<Entity, With<TitlescreenParent>>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (r#loop, boop_moon, hoop_moon) = spawn_loop(
        LoopInfo {
            position: PLAY_BTN_LOCATION.clone(),
            planet: Planet::One,
            boop_prices: FIRST_PLANET_BOOP_PRICES.clone(),
            hoop_prices: FIRST_PLANET_HOOP_PRICES.clone(),
        },
        &mut commands,
        &asset_server,
    );

    commands
        .entity(r#loop)
        .entry::<Transform>()
        .and_modify(|mut t| {
            t.scale *= FIRST_PLANET_INITIAL_SCALE;
            t.translation.z = -2.;
        });

    commands.entity(*titlescreen_moon).despawn();

    commands
        .entity(boop_moon)
        .insert_recursive::<Children>(Disabled);
    commands
        .entity(hoop_moon)
        .insert_recursive::<Children>(Disabled);

    commands
        .entity(*titlescreen_btn)
        .insert(Animator::new(fade_to_transparent()));

    commands
        .entity(*titlescreen_art)
        .insert(Animator::new(fade_to_transparent()));

    commands
        .entity(*titlescreen_parent)
        .insert(Animator::new(Tracks::new([move_titlescreen_with_planet()])));

    commands
        .entity(r#loop)
        .insert(Animator::new(center_first_planet().then(scale_planet_up())));
}
