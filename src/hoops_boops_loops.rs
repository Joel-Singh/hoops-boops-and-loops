/// This module handles the core logic of each Loop. Note that a "r#" had to be prepended when using
/// loop because its a keyword
use crate::loot::Loot;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Component)]
struct Boop {
    r#loop: Entity,
    /// In Radians from 0 to 2*PI
    current_loop_position: f32,
    in_hoop: bool,
}

impl Default for Boop {
    fn default() -> Self {
        Boop {
            r#loop: Entity::PLACEHOLDER,
            current_loop_position: 0.,
            in_hoop: false,
        }
    }
}

#[derive(Component)]
struct Hoop {
    r#loop: Entity,
}

/// Corresponds to the different planet sprites since each hoop is colored to their specific planet
#[derive(Copy, Clone)]
pub enum Planet {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Planet {
    fn get_number(&self) -> String {
        match self {
            Planet::One => "1".to_string(),
            Planet::Two => "2".to_string(),
            Planet::Three => "3".to_string(),
            Planet::Four => "4".to_string(),
            Planet::Five => "5".to_string(),
            Planet::Six => "6".to_string(),
        }
    }

    fn get_sprite_path(&self) -> String {
        return "loop-".to_string() + &self.get_number() + &".png";
    }

    fn get_inner_hoop_path(&self, count: i32) -> String {
        let number = self.get_number();
        return "hoops/loop-".to_string()
            + &number
            + &"/inner-half-"
            + &count.to_string()
            + &".png";
    }

    fn get_outer_hoop_path(&self, count: i32) -> String {
        let number = self.get_number();
        return "hoops/loop-".to_string()
            + &number
            + &"/outer-half-"
            + &count.to_string()
            + &".png";
    }
}

#[derive(Component)]
struct Loop {
    boops: Vec<Entity>,
    hoop_count: i32,
    planet: Planet,
}

/// The max number of hoops
const MAX_HOOPS: i32 = 8;
const MAX_BOOPS: usize = 16;

const LOOP_FILE_HEIGHT: f32 = 295.;
const LOOP_RADIUS: f32 = LOOP_FILE_HEIGHT / 2.;

const BOOP_TO_LOOP_MARGIN: f32 = 10.;
const DEFAULT_BOOP_TRANSLATION: Vec3 = Vec3::new(0., LOOP_RADIUS + BOOP_TO_LOOP_MARGIN, 0.);

pub fn hoops_boops_loops_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (move_boops_forward, position_boops, get_loot_on_boop_in_hoop).chain(),
    );
}

/// Positions boops according to Boop::current_loop_position
fn position_boops(boop_q: Query<(&mut Transform, &Boop)>) {
    for boop in boop_q {
        let mut transform = boop.0;
        let boop = boop.1;

        transform.translation = DEFAULT_BOOP_TRANSLATION;
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(boop.current_loop_position),
        );
    }
}

/// Moves boops forwards by incrementing Boop::current_loop_position, modulating it to keep it
/// between 0 and 2PI
fn move_boops_forward(boops: Query<&mut Boop>, time: Res<Time>) {
    const BOOP_SPEED: f32 = 1.2;
    for mut boop in boops {
        let increase = BOOP_SPEED * time.delta_secs();
        boop.current_loop_position += increase;
        boop.current_loop_position %= 2. * PI;
        boop.current_loop_position;
    }
}

/// Increments loot by 1 whenever a boop enters a hoop
fn get_loot_on_boop_in_hoop(
    mut boop_q: Query<(&Transform, &mut Boop)>,
    loop_q: Query<&Loop>,
    mut loot: ResMut<Loot>,
) {
    // counterclockwise from the top left
    let hoop_positions: [Vec2; 8] = [
        (-64.5, 140.5).into(),
        (-147.5, 55.5).into(),
        (-146.0, -58.5).into(),
        (-67.5, -143.5).into(),
        (66.5, -142.5).into(),
        (148.5, -56.5).into(),
        (147.5, 56.5).into(),
        (66.5, 141.5).into(),
    ];

    let on_hoop_tolerance = 5.;

    for r#loop in loop_q {
        for boop in r#loop.boops.clone() {
            let (boop_trans, mut boop) = boop_q.get_mut(boop).unwrap();

            let mut in_any_hoop = false;
            for i in 0..r#loop.hoop_count {
                let currently_in_hoop = boop_trans
                    .translation
                    .truncate()
                    .distance(hoop_positions[i as usize])
                    <= on_hoop_tolerance;
                if currently_in_hoop {
                    in_any_hoop = true;
                    break;
                }
            }

            if in_any_hoop && !boop.in_hoop {
                **loot += 1;
                boop.in_hoop = true;
            }

            if !in_any_hoop {
                boop.in_hoop = false;
            }
        }
    }
}

pub struct SpawnLoop {
    pub position: Vec2,
    pub planet: Planet,
}

impl Command for SpawnLoop {
    // Spawns a loop with with 1 boop and 1 hoop at the position of SpawnLoop.0
    // Hoops and Loops will be children of the loop (along with being kept track of in Loops.boops/Loops.hoops) so that their transform is relative to the Loop

    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let loop_image = asset_server.load(self.planet.get_sprite_path());

        let mut commands = world.commands();

        const LOOP_SCALE: f32 = 0.8;

        let r#loop = commands
            .spawn((
                Sprite::from_image(loop_image),
                Transform {
                    translation: self.position.extend(0.0),
                    scale: Transform::default().scale * LOOP_SCALE,
                    ..default()
                },
                Loop {
                    boops: Vec::default(),
                    hoop_count: 0,
                    planet: self.planet,
                },
                ZIndex(-2),
            ))
            .id();

        commands.queue(AddBoop(r#loop));
        commands.queue(AddHoop(r#loop));
        commands.queue(AddHoop(r#loop));
        commands.queue(AddHoop(r#loop));
        commands.queue(AddHoop(r#loop));
    }
}

/// Custom EntityCommand that adds a hoop to a loop
/// panics if you try to add a hoop to a loop that already has MAX_HOOPS or if the entity does not
/// contain the Loop Component
pub struct AddHoop(Entity);
impl Command for AddHoop {
    fn apply(self, world: &mut World) {
        let r#loop = world.entity_mut(self.0);

        let r#loop_component = r#loop.get::<Loop>().unwrap();
        let hoop_count = r#loop_component.hoop_count;
        let planet = r#loop_component.planet;

        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        let outer_hoop_image = asset_server.load(planet.get_outer_hoop_path(hoop_count + 1));
        let inner_hoop_image = asset_server.load(planet.get_inner_hoop_path(hoop_count + 1));

        let outer_hoop = world
            .spawn((
                Sprite {
                    image: outer_hoop_image,
                    ..default()
                },
                Transform {
                    translation: Vec3::new(0., 0., 1.),
                    ..default()
                },
            ))
            .id();

        let inner_hoop = world
            .spawn((
                Sprite {
                    image: inner_hoop_image,
                    ..default()
                },
                Transform {
                    translation: Vec3::new(0., 0., -1.),
                    ..default()
                },
            ))
            .id();

        let mut r#loop = world.entity_mut(self.0);
        r#loop.add_child(outer_hoop);
        r#loop.add_child(inner_hoop);

        let mut r#loop = r#loop.get_mut::<Loop>().unwrap();
        if r#loop.hoop_count >= MAX_HOOPS {
            panic!("Added a hoop to a loop that already has max hoops");
        }
        r#loop.hoop_count += 1;
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

        const BOOP_SCALE: f32 = 0.03;

        let new_boop = world
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

        let mut r#loop = world.entity_mut(r#loop);
        r#loop.add_child(new_boop);

        let mut r#loop = r#loop.get_mut::<Loop>().unwrap();
        let boop_count = r#loop.boops.len();

        if boop_count >= MAX_BOOPS {
            panic!("Added a boop to a loop that already has max boops");
        }

        r#loop.boops.push(new_boop);
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
