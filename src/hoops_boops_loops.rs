/// This module handles the core logic of each Loop. Note that a "r#" had to be prepended when using
/// loop because its a keyword
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

#[derive(Component)]
struct Boop {
    r#loop: Entity,
    current_loop_position: Rot2,
    in_hoop: bool,
}

#[derive(Component)]
struct Hoop {
    r#loop: Entity,
}

#[derive(Component)]
struct Loop {
    boops: Vec<Entity>,
    hoops: Vec<Entity>,
}

const MAX_HOOPS: u32 = 8;
/// The max number of hoop
const INITIAL_HOOP_POSITION: Rot2 = Rot2::FRAC_PI_8;
/// The initial position of the first hoop.

pub fn hoops_boops_loops_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (move_boops_forward, position_boops, get_loot_on_boop_in_hoop).chain(),
    );
}

/// Positions boops according to Boop::current_loop_position
fn position_boops() {}

/// Moves boops forwards by incrementing Boop::current_loop_percentage
fn move_boops_forward() {}

/// Increments loot by 1 whenever a boop enters a hoop
fn get_loot_on_boop_in_hoop() {}

pub struct SpawnLoop(pub Vec2);

impl Command for SpawnLoop {
    // Spawns a loop with with 1 boop and 1 hoop at the position of SpawnLoop.0
    // Hoops and Loops will be children of the loop (along with being kept track of in Loops.boops/Loops.hoops) so that their transform is relative to the Loop

    fn apply(self, world: &mut World) {}
}

/// Custom EntityCommand that adds a hoop to a loop
/// The hoop is placed in the INITIAL_HOOP_POSITION, or else, 1/8 from the last one
/// Does nothing and emits warning if you try to add a hoop to a loop that already has 8
pub struct AddHoop;
impl EntityCommand for AddHoop {
    fn apply(self, entity: EntityWorldMut) {}
}

pub struct AddBoop;
impl EntityCommand for AddBoop {
    fn apply(self, entity: EntityWorldMut) {}
}

/// Load randomly "file-#.svg" with the # being replaced by a random number from range.
fn load_random_variant(
    file: String,
    range: Range<u32>,
    asset_server: &AssetServer,
) -> Handle<Image> {
    let mut rng = rand::rng();
    asset_server.load("file-".to_string() + &rng.random_range(range).to_string() + ".svg")
}
