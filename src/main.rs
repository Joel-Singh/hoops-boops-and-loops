use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

mod background;
mod buy_boops_and_hoops;
mod hoops_boops_loops;
mod loot;

use background::background_plugin;
use buy_boops_and_hoops::buy_boops_and_hoops_plugin;
use hoops_boops_loops::{Planet, SpawnLoop, hoops_boops_loops_plugin};
use loot::loot_plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            hoops_boops_loops_plugin,
            background_plugin,
            loot_plugin,
            buy_boops_and_hoops_plugin,
            TweeningPlugin,
        ))
        .add_systems(Startup, (setup_camera, spawn_loop))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_loop(mut commands: Commands) {
    commands.queue(SpawnLoop {
        position: Vec2::ZERO,
        planet: Planet::Two,
        // Prices can ONLY be whole tens, hundreds, or thousands for display purposes (see buy_boops_and_hoops::i32_to_display_str)
        boop_prices: [
            1, 5, 10, 20, 30, 50, 80, 100, 200, 300, 400, 500, 600, 700, 800,
        ],
        hoop_prices: [1, 5, 10, 20, 30, 50, 100, 200],
    });
}
