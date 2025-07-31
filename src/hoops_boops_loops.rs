/// This module handles the core logic of each Loop. Note that a "r#" had to be prepended when using
/// loop because its a keyword
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Boop {
    r#loop: Entity,
    current_loop_position: Rot2,
    in_hoop: bool,
}

impl Default for Boop {
    fn default() -> Self {
        Boop {
            r#loop: Entity::PLACEHOLDER,
            current_loop_position: Rot2::default(),
            in_hoop: false,
        }
    }
}

#[derive(Component)]
struct Hoop {
    r#loop: Entity,
}

#[derive(Component, Default)]
struct Loop {
    boops: Vec<Entity>,
    hoops: Vec<Entity>,
}

const MAX_HOOPS: u32 = 8;
const MAX_BOOPS: u32 = 16;
/// The max number of hoop
const INITIAL_HOOP_POSITION: Rot2 = Rot2::FRAC_PI_8;
/// The initial position of the first hoop.

const LOOP_FILE_HEIGHT: f32 = 720.;
const LOOP_RADIUS: f32 = LOOP_FILE_HEIGHT / 2.;

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

    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let loop_image = load_random_variant("loop", &asset_server, 1, 5);

        let mut commands = world.commands();

        const LOOP_SCALE: f32 = 0.15;

        let r#loop = commands
            .spawn((
                Sprite::from_image(loop_image),
                Transform {
                    translation: self.0.extend(0.0),
                    scale: Transform::default().scale * LOOP_SCALE,
                    ..default()
                },
                Loop::default(),
            ))
            .id();

        commands.queue(AddBoop(r#loop));
    }
}

/// Custom EntityCommand that adds a hoop to a loop
/// The hoop is placed in the INITIAL_HOOP_POSITION, or else, 1/8 from the last one
/// Does nothing and emits warning if you try to add a hoop to a loop that already has MAX_HOOPS. Or if
/// this is ran on an entity without the Loop component
pub struct AddHoop;
impl EntityCommand for AddHoop {
    fn apply(self, entity: EntityWorldMut) {}
}

/// Custom EntityCommand that adds a boop to a loop
/// panics if you try to add a boop to a loop that already has MAX_BOOPS
/// Does not check if entity is a loop, behavior is undefined if so
pub struct AddBoop(Entity);
impl Command for AddBoop {
    fn apply(self, world: &mut World) {
        let r#loop = self.0;
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let boop_image = load_random_variant("boop", &asset_server, 1, 5);

        let mut commands = world.commands();

        const BOOP_SCALE: f32 = 0.1;
        const BOOP_TO_LOOP_MARGIN: f32 = 70.;

        let new_boop = commands
            .spawn((
                Sprite::from_image(boop_image),
                Transform {
                    translation: Vec3::default().with_y(LOOP_RADIUS + BOOP_TO_LOOP_MARGIN),
                    scale: Transform::default().scale * BOOP_SCALE,
                    ..default()
                },
                Boop {
                    r#loop,
                    ..default()
                },
            ))
            .id();

        commands
            .entity(r#loop)
            .entry::<Loop>()
            .and_modify(move |mut r#loop| r#loop.boops.push(new_boop));

        commands.entity(r#loop).add_child(new_boop);
    }
}

/// Load randomly "file-#.svg" with the # being replaced by a random number from start to end
/// inclusive on both ends
fn load_random_variant(
    file_name: &'static str,
    asset_server: &AssetServer,
    start: u32,
    end: u32,
) -> Handle<Image> {
    let mut rng = rand::rng();
    asset_server
        .load(file_name.to_owned() + "-" + &rng.random_range(start..=end).to_string() + ".png")
}
