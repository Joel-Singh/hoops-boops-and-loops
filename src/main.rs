use bevy::prelude::*;

mod hoops_boops_loops;
use hoops_boops_loops::{SpawnLoop, hoops_boops_loops_plugin};

mod background;
use background::background_plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_svg::prelude::SvgPlugin,
            hoops_boops_loops_plugin,
            background_plugin,
        ))
        .add_systems(Startup, (setup_camera, spawn_loop))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_loop(mut commands: Commands) {
    commands.queue(SpawnLoop(Vec2::ZERO));
}
