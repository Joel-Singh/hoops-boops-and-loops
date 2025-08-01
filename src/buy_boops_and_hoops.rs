use crate::hoops_boops_loops::{AddBoop, AddHoop, Orbit, Planet};
use crate::loot::Loot;
use bevy::audio::PlaybackMode;
use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;
use bevy_tweening::Animator;
use bevy_tweening::RepeatCount;
use bevy_tweening::RepeatStrategy;
use bevy_tweening::Tween;
use bevy_tweening::lens::SpriteColorLens;
use std::f32::consts::PI;
use std::time::Duration;

/// The buttons for buying new hoops and boops
#[derive(Component)]
struct MoonBtn {
    /// Goes to next price after buying,
    price_list: Vec<i32>,
    /// corresponds to an index in price_list
    current_price: usize,
    /// The text that displays the price
    text: Entity,
    /// The loop that this btn buys for
    r#loop: Entity,
}

#[derive(Component)]
/// Marker struct for Text2ds that show the price
struct PriceText;

const BUY_BOOP_STARTING_ORBIT: f32 = 0.30;

pub fn buy_boops_and_hoops_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, advance_moon_btn_orbits);
}

/// Creates a moon button that buys boops
pub fn create_buy_boop_button(r#loop: Entity, boop_prices: [i32; 15], world: &mut World) {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let moon_image = asset_server.load("moon-btn.png");
    let rocket_image = asset_server.load("buy-boop-showcase.png");
    let loot_symbol_image = asset_server.load("loot-symbol.png");
    let spacey_font = asset_server.load("SpaceGrotesk-Light.ttf");

    let buy_boop_btn = world
        .spawn((
            Sprite::from_image(moon_image),
            Orbit {
                current_loop_position: BUY_BOOP_STARTING_ORBIT,
                starting_transform: Transform {
                    translation: Vec3::new(0., 300., 0.),
                    ..default()
                },
            },
            Pickable::default(),
        ))
        .id();

    let buy_boop_showcase = world
        .spawn((
            Sprite::from_image(rocket_image),
            Transform {
                translation: Vec3::new(0., 60., 1.),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    const LOOT_SYMBOL_SCALE: f32 = 0.05;
    let loot_symbol = world
        .spawn((
            Sprite::from_image(loot_symbol_image),
            Transform {
                translation: Vec3::new(-20., 0., 1.),
                scale: Vec3::splat(LOOT_SYMBOL_SCALE),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    let text = world
        .spawn((
            Text2d::new(i32_to_display_str(boop_prices[0])),
            TextFont {
                font: spacey_font,
                font_size: 40.,
                ..default()
            },
            TextColor(BLACK.into()),
            Transform {
                translation: Vec3::new(20., 0., 0.),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    world
        .entity_mut(buy_boop_btn)
        .add_children(&[buy_boop_showcase, loot_symbol, text])
        .insert(MoonBtn {
            price_list: boop_prices.to_vec(),
            current_price: 0,
            text,
            r#loop,
        })
        .observe(buy_new_x_on_click::<AddBoop>);

    world.entity_mut(r#loop).add_child(buy_boop_btn);
}

/// Creates a hoop button that buys hoops
pub fn create_buy_hoop_button(
    r#loop: Entity,
    planet: Planet,
    hoop_prices: [i32; 7],
    world: &mut World,
) {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let moon_image = asset_server.load("moon-btn.png");
    let hoop_showcase = asset_server.load(planet.get_hoop_showcase_path());
    let loot_symbol_image = asset_server.load("loot-symbol.png");
    let spacey_font = asset_server.load("SpaceGrotesk-Light.ttf");

    let buy_hoop_btn = world
        .spawn((
            Sprite::from_image(moon_image),
            Orbit {
                current_loop_position: BUY_BOOP_STARTING_ORBIT + PI,
                starting_transform: Transform {
                    translation: Vec3::new(0., 300., 0.),
                    ..default()
                },
            },
            Pickable::default(),
        ))
        .id();

    let buy_hoop_showcase = world
        .spawn((
            Sprite::from_image(hoop_showcase),
            Transform {
                translation: Vec3::new(0., 60., 1.),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    const LOOT_SYMBOL_SCALE: f32 = 0.05;
    let loot_symbol = world
        .spawn((
            Sprite::from_image(loot_symbol_image),
            Transform {
                translation: Vec3::new(-20., 0., 1.),
                scale: Vec3::splat(LOOT_SYMBOL_SCALE),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    let text = world
        .spawn((
            Text2d::new(i32_to_display_str(hoop_prices[0])),
            TextFont {
                font: spacey_font,
                font_size: 40.,
                ..default()
            },
            TextColor(BLACK.into()),
            Transform {
                translation: Vec3::new(20., 0., 0.),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    world
        .entity_mut(buy_hoop_btn)
        .add_children(&[buy_hoop_showcase, loot_symbol, text])
        .insert(MoonBtn {
            price_list: hoop_prices.to_vec(),
            current_price: 0,
            text,
            r#loop,
        })
        .observe(buy_new_x_on_click::<AddHoop>);

    world.entity_mut(r#loop).add_child(buy_hoop_btn);
}

/// Advance the moon btns orbit, wrapping at 2. * PI
fn advance_moon_btn_orbits(btns: Query<&mut Orbit, With<MoonBtn>>, time: Res<Time>) {
    const MOON_BTN_SPEED: f32 = 0.05;
    for mut orbit in btns {
        let increase = MOON_BTN_SPEED * time.delta_secs();
        orbit.current_loop_position += increase;
        orbit.current_loop_position %= 2. * PI;
    }
}

// If enough loot, decrements loot, queues the T command which adds the new thing bought, and updates the price text. Otherwise, makes a little *err* sound and turns orange briefly.
fn buy_new_x_on_click<T: Command>(
    trigger: Trigger<Pointer<Click>>,
    mut loot: ResMut<Loot>,
    mut moon_btn_q: Query<(&mut MoonBtn, Entity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    T: From<Entity>,
{
    let (mut moon_btn, moon_btn_e) = moon_btn_q.get_mut(trigger.target).unwrap();

    let current_price = moon_btn.price_list[moon_btn.current_price];
    if **loot >= current_price {
        **loot -= current_price;
        moon_btn.current_price += 1;

        commands.spawn((
            AudioPlayer::new(asset_server.load("successful_buy.wav")),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        ));
        commands.queue(T::from(moon_btn.r#loop));

        let new_price = moon_btn.price_list[moon_btn.current_price];
        commands
            .entity(moon_btn.text)
            .insert(Text2d::new(i32_to_display_str(new_price)));

        let fade_in_blue_tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(0.5),
            SpriteColorLens {
                start: Color::WHITE,
                end: Srgba::rgb(0.333, 0.808, 0.929).into(),
            },
        )
        .with_repeat_count(RepeatCount::Finite(2))
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

        commands
            .entity(moon_btn_e)
            .insert(Animator::new(fade_in_blue_tween));
    } else {
        let fade_in_orange_tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(0.5),
            SpriteColorLens {
                start: Color::WHITE,
                end: Srgba::rgb(1.0, 0.604, 0.259).into(),
            },
        )
        .with_repeat_count(RepeatCount::Finite(2))
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

        commands.spawn((
            AudioPlayer::new(asset_server.load("unsuccessful_buy.wav")),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        ));

        commands
            .entity(moon_btn_e)
            .insert(Animator::new(fade_in_orange_tween));
    }
}

/// Converts i32 to a string to be displayed on the moon btns for price. Only supports numbers with
/// a digit with leading zeroes and single digits up to 99,000. Will panic otherwise.
/// Uses h and k for abbreviations of 100 and 1000 respectively.
fn i32_to_display_str(num: i32) -> String {
    if num < 10 {
        return num.to_string();
    }

    if num > 99000 {
        panic!("Tried to display too big of a number");
    }

    if num % 10 == 0 {
        return num.to_string();
    }

    if num % 100 == 0 {
        return (num / 100).to_string() + "h";
    }

    if num % 1000 == 0 {
        return (num / 1000).to_string() + "k";
    }

    panic!("Tried to display unsupported number");
}
