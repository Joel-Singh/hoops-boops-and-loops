use crate::hoops_boops_loops::Planet;
use bevy::prelude::*;

#[derive(Component)]
pub struct LockedPlanet;

pub fn spawn_locked_planet(
    planet: Planet,
    pos: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    let img = asset_server.load(planet.get_locked_path());
    commands
        .spawn((
            Sprite::from_image(img),
            Transform {
                translation: pos.extend(0.),
                ..default()
            },
            LockedPlanet,
        ))
        .id()
}
