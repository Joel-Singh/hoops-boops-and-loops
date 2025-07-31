/// This module handles the core logic of each Loop. Note that a "r#" had to be prepended when using
/// loop because its a keyword
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use std::f32::consts::PI;

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

const MAX_HOOPS: usize = 8;
const MAX_BOOPS: usize = 16;
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
        commands.queue(AddHoop(r#loop));
    }
}

/// Custom EntityCommand that adds a hoop to a loop
/// The hoop is placed in the INITIAL_HOOP_POSITION, or else, 1/8 from the last one
/// panics if you try to add a hoop to a loop that already has MAX_HOOPS or if the entity does not
/// contain the Loop Component
pub struct AddHoop(Entity);
impl Command for AddHoop {
    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let hoop_image = load_random_variant("hoop", &asset_server, 1, 5);
        let r#loop = self.0;

        const HOOP_SCALE: f32 = 0.3;
        const HOOP_TO_LOOP_MARGIN: f32 = -10.;

        let new_hoop = world
            .spawn((
                Sprite {
                    image: hoop_image,
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform {
                    translation: Vec3::default().with_y(LOOP_RADIUS + HOOP_TO_LOOP_MARGIN),
                    scale: Transform::default().scale * HOOP_SCALE,
                    ..default()
                },
                Hoop { r#loop },
            ))
            .id();

        let mut r#loop = world.entity_mut(self.0);
        r#loop.add_child(new_hoop);

        let mut r#loop = r#loop.get_mut::<Loop>().unwrap();
        let hoop_count = r#loop.hoops.len();
        if hoop_count >= MAX_HOOPS {
            panic!();
        }
        r#loop.hoops.push(new_hoop);

        let mut new_hoop_transform = world.get_mut::<Transform>(new_hoop).unwrap();
        new_hoop_transform.rotation =
            Quat::from_rotation_z(2. * PI * (hoop_count as f32 / MAX_HOOPS as f32));
    }
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
            .and_modify(move |mut r#loop| {
                if r#loop.boops.len() >= MAX_BOOPS {
                    panic!();
                }
                r#loop.boops.push(new_boop);
            });

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
