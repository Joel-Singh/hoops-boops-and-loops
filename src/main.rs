use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_svg::prelude::SvgPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
