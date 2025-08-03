use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_tweening::TweeningPlugin;

mod background;
mod buy_boops_and_hoops;
mod hoops_boops_loops;
mod locked_planets;
mod loot;
mod orbit_starting_transform_y_lens;
mod play_hoop_through_boop_sounds;
mod prices;
mod projection_scale_lens;
mod scales;
mod screen_size;
mod soundtrack;
mod titlescreen;
mod transition_to_all_planets;
mod transition_to_first_planet;

use crate::orbit_starting_transform_y_lens::orbit_starting_transform_y_lens_plugin;
use background::background_plugin;
use buy_boops_and_hoops::buy_boops_and_hoops_plugin;
use hoops_boops_loops::hoops_boops_loops_plugin;
use loot::loot_plugin;
use projection_scale_lens::projection_scale_lens_plugin;
use screen_size::SCREEN_SIZE;
use soundtrack::soundtrack_plugin;
use titlescreen::titlescreen_plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(SCREEN_SIZE.x, SCREEN_SIZE.y),
                    title: "Hoops & Boops and don't forget about loops!".to_string(),
                    ..default()
                }),
                ..default()
            }),
            hoops_boops_loops_plugin,
            background_plugin,
            loot_plugin,
            buy_boops_and_hoops_plugin,
            TweeningPlugin,
            soundtrack_plugin,
            titlescreen_plugin,
            projection_scale_lens_plugin,
            orbit_starting_transform_y_lens_plugin,
            transition_to_all_planets::plugin,
            locked_planets::plugin,
            play_hoop_through_boop_sounds::plugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
