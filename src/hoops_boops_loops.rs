/// This module handles the core logic of each Loop. Note that a "r#" had to be prepended when using
/// loop because its a keyword
use crate::buy_boops_and_hoops::{create_buy_boop_button, create_buy_hoop_button};
use crate::loot::Loot;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy_tweening::Animator;
use bevy_tweening::RepeatCount;
use bevy_tweening::RepeatStrategy;
use bevy_tweening::Tween;
use bevy_tweening::lens::SpriteColorLens;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Component)]
struct Boop {
    r#loop: Entity,
    in_hoop: bool,
}

/// Rotates entities in an orbit around their Transform origin (which could be their parents)
/// from `starting_transform`
/// REMEMBER, the transform will be reset the starting_transform on each tick
#[derive(Component)]
pub struct Orbit {
    /// In Radians from 0 to 2*PI
    pub current_loop_position: f32,
    pub starting_transform: Transform, // Will determine how far away the entitiy will be
}

impl Default for Boop {
    fn default() -> Self {
        Boop {
            r#loop: Entity::PLACEHOLDER,
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

    /// We need a different hoop showcase for each planet because they are colored for the
    /// individual planet
    pub fn get_hoop_showcase_path(&self) -> String {
        let number = self.get_number();
        return "buy-hoop-showcase/".to_string() + &number + &".png";
    }
}

#[derive(Component)]
struct Loop {
    boops: Vec<Entity>,
    hoop_count: i32,
    /// The hoop_sprites.0 is outer and hoop_sprites.1 is inner
    hoop_sprites: Vec<(Entity, Entity)>,
    planet: Planet,
}

/// The max number of hoops
const MAX_HOOPS: i32 = 8;
const MAX_BOOPS: usize = 16;

const LOOP_FILE_HEIGHT: f32 = 295.;
const LOOP_RADIUS: f32 = LOOP_FILE_HEIGHT / 2.;

pub fn hoops_boops_loops_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (move_boops_forward, orbit, get_loot_on_boop_in_hoop).chain(),
    );
}

/// Positions the transform of an orbit according to Orbit::current_loop_position
fn orbit(orbit_q: Query<(&mut Transform, &Orbit)>) {
    for orbit in orbit_q {
        let mut transform = orbit.0;
        let orbit = orbit.1;

        *transform = orbit.starting_transform;
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(orbit.current_loop_position),
        );
    }
}

/// Moves boops forwards by incrementing their Orbit::current_loop_position modulating it to keep it
/// between 0 and 2PI
fn move_boops_forward(boops: Query<&mut Orbit, With<Boop>>, time: Res<Time>) {
    const BOOP_SPEED: f32 = 1.2;
    for mut orbit in boops {
        let increase = BOOP_SPEED * time.delta_secs();
        orbit.current_loop_position += increase;
        orbit.current_loop_position %= 2. * PI;
        orbit.current_loop_position;
    }
}

/// Increments loot by 1 whenever a boop enters a hoop
fn get_loot_on_boop_in_hoop(
    mut boop_q: Query<(&Transform, &mut Boop)>,
    loop_q: Query<&Loop>,
    mut loot: ResMut<Loot>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

    let on_hoop_tolerance = 10.;

    for r#loop in loop_q {
        for boop in r#loop.boops.clone() {
            let (boop_trans, mut boop) = boop_q.get_mut(boop).unwrap();

            // i32 represents an index in Loop::hoop_sprites
            let mut in_hoop: Option<i32> = None;
            for i in 0..r#loop.hoop_count {
                let currently_in_hoop = boop_trans
                    .translation
                    .truncate()
                    .distance(hoop_positions[i as usize])
                    <= on_hoop_tolerance;
                if currently_in_hoop {
                    in_hoop = Some(i);
                    break;
                }
            }

            if in_hoop.is_some() && !boop.in_hoop {
                let in_hoop = in_hoop.unwrap();

                **loot += 1;
                boop.in_hoop = true;

                commands
                    .entity(r#loop.hoop_sprites[in_hoop as usize].0)
                    .insert(Animator::new(brief_fade_to_white_tween()));

                commands
                    .entity(r#loop.hoop_sprites[in_hoop as usize].1)
                    .insert(Animator::new(brief_fade_to_white_tween()));

                commands.spawn((
                    AudioPlayer::new(asset_server.load("boop-going-through-hoop.ogg")),
                    PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                ));
            }

            if in_hoop.is_none() {
                boop.in_hoop = false;
            }
        }
    }
}

pub struct LoopInfo {
    pub position: Vec2,
    pub planet: Planet,
    pub boop_prices: [i32; 15],
    pub hoop_prices: [i32; 8],
}

/// Spawns a loop, returning the loop, buy boop btn, and buy hoop btn, in that order
pub fn spawn_loop(
    loop_info: LoopInfo,
    mut commands: &mut Commands,
    asset_server: &AssetServer,
) -> (Entity, Entity, Entity) {
    let loop_image = asset_server.load(loop_info.planet.get_sprite_path());

    let r#loop = commands
        .spawn((
            Sprite::from_image(loop_image),
            Transform {
                translation: loop_info.position.extend(0.0),
                ..default()
            },
            Loop {
                boops: Vec::default(),
                hoop_count: 0,
                hoop_sprites: Vec::default(),
                planet: loop_info.planet,
            },
            ZIndex(-2),
        ))
        .id();

    commands.queue(AddBoop(r#loop));
    commands.queue(AddHoop(r#loop));

    let boop = create_buy_boop_button(r#loop, loop_info.boop_prices, &mut commands, asset_server);
    let hoop = create_buy_hoop_button(
        r#loop,
        loop_info.planet,
        loop_info.hoop_prices,
        &mut commands,
        asset_server,
    );

    (r#loop, boop, hoop)
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
        r#loop.hoop_sprites.push((outer_hoop, inner_hoop));
    }
}

// Used to allow for use in a generic
impl From<Entity> for AddHoop {
    fn from(entity: Entity) -> Self {
        AddHoop(entity)
    }
}

/// Custom EntityCommand that adds a boop to a loop
/// panics if you try to add a boop to a loop that already has MAX_BOOPS
/// Does not check if entity is a loop, behavior is undefined if so
pub struct AddBoop(pub Entity);
impl Command for AddBoop {
    fn apply(self, world: &mut World) {
        let r#loop = self.0;
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let boop_image = load_random_variant("boop", &asset_server, 1, 5);

        const BOOP_SCALE: f32 = 0.3;
        const BOOP_TO_LOOP_MARGIN: f32 = 12.;

        let starting_transform = Transform {
            translation: Vec3::new(0., LOOP_RADIUS + BOOP_TO_LOOP_MARGIN, 0.),
            scale: Transform::default().scale * BOOP_SCALE,
            ..default()
        };

        let new_boop = world
            .spawn((
                Sprite::from_image(boop_image),
                starting_transform,
                Boop {
                    r#loop,
                    ..default()
                },
                Orbit {
                    current_loop_position: 0.,
                    starting_transform,
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

// Used to allow for use in a generic
impl From<Entity> for AddBoop {
    fn from(entity: Entity) -> Self {
        AddBoop(entity)
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

fn brief_fade_to_white_tween() -> Tween<Sprite> {
    Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(0.5),
        SpriteColorLens {
            start: Color::WHITE,
            end: Srgba::rgb_u8(237, 235, 202).into(),
        },
    )
    .with_repeat_count(RepeatCount::Finite(2))
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
}
