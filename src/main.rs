use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;

mod hoops_boops_loops;
use hoops_boops_loops::{SpawnLoop, hoops_boops_loops_plugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_svg::prelude::SvgPlugin,
            hoops_boops_loops_plugin,
        ))
        .add_systems(Startup, (setup_camera, set_background_color, spawn_loop))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn set_background_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BLACK.into()
}

fn spawn_loop(mut commands: Commands) {
    commands.queue(SpawnLoop(Vec2::ZERO));
}
