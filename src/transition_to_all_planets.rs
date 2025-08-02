mod tweens;

use crate::hoops_boops_loops::AllHoopsBought;
use crate::hoops_boops_loops::Planet;
use crate::locked_planets::{LockedPlanet, spawn_locked_planet};
use crate::screen_size::SCREEN_SIZE;
use crate::transition_to_first_planet::FirstPlanet;
use bevy::prelude::*;
use bevy_tweening::Animator;
use std::iter::zip;
use tweens::*;

#[derive(Resource, Deref, DerefMut)]
struct Transitioned(bool);

pub fn plugin(app: &mut App) {
    app.insert_resource(Transitioned(false))
        .add_observer(transition_to_all_planets_on_all_hoops_bought);
}

const PLANET_COUNT: i32 = 6;
fn transition_to_all_planets(
    first_planet: Single<Entity, With<FirstPlanet>>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let planet_positions = calculate_planet_positions();

    // Transition first planet to a smaller size and to the proper size and the generated
    // position
    let tween_planet_scales = commands.register_system(tween_planet_scales);
    commands.entity(*first_planet).insert(Animator::new(
        move_first_planet(planet_positions[0].extend(0.0))
            .with_completed_system(tween_planet_scales),
    ));

    for (i, planet) in zip(
        1..PLANET_COUNT,
        [
            Planet::Two,
            Planet::Three,
            Planet::Four,
            Planet::Five,
            Planet::Six,
        ],
    ) {
        let locked_planet = spawn_locked_planet(
            planet,
            planet_positions[i as usize],
            &mut commands,
            &asset_server,
        );

        commands
            .entity(locked_planet)
            .entry::<Transform>()
            .and_modify(|mut t| t.scale = Vec3::ZERO);
    }

    fn calculate_planet_positions() -> [Vec2; 6] {
        let margin: Vec2 = Vec2::splat(200.);

        let available_space: Vec2 = SCREEN_SIZE - (margin * 2.);
        let planet_x_spacing: f32 = available_space.x / 2.;

        // 2 rows
        let available_row_space: f32 = available_space.y / 2.;

        let row_gap: f32 = 80.;
        let row_one_center_y: f32 = available_row_space / 2. + row_gap;

        let row_two_center_y: f32 = -row_one_center_y;

        let mut planet_positions: [Vec2; 6] = [Vec2::default(); 6];

        let x_spacing = |i: i32| {
            // Subtract by SCREEN_SIZE to make it from the left screen
            margin.x + planet_x_spacing * (i as f32) - (SCREEN_SIZE.x / 2.)
        };

        for i in 0..(PLANET_COUNT / 2) {
            planet_positions[i as usize] = Vec2::new(x_spacing(i), row_one_center_y);
        }

        for i in (PLANET_COUNT / 2)..PLANET_COUNT {
            planet_positions[i as usize] = Vec2::new(x_spacing(i - 3), row_two_center_y);
        }

        planet_positions
    }
}

fn transition_to_all_planets_on_all_hoops_bought(
    _: Trigger<AllHoopsBought>,
    mut commands: Commands,
    mut transitioned: ResMut<Transitioned>,
) {
    if **transitioned == false {
        commands.run_system_cached(transition_to_all_planets);
        **transitioned = true;
    }
}

fn tween_planet_scales(
    mut commands: Commands,
    planets_q: Query<Entity, With<LockedPlanet>>,
    first_planet: Single<Entity, With<FirstPlanet>>,
) {
    for planet in planets_q {
        commands
            .entity(planet)
            .insert(Animator::new(zoom_planet_in_from_zero()));
    }

    commands
        .entity(*first_planet)
        .insert(Animator::new(zoom_planet_out()));
}
