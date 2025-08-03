use crate::hoops_boops_loops::{AddBoop, AddHoop, Orbit, Planet};
use crate::loot::Loot;
use crate::scales::*;
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
    current_price_index: usize,
    /// The text that displays the price
    text: Entity,
    /// The loop that this btn buys for
    r#loop: Entity,
}

impl MoonBtn {
    fn reached_max_buy_amount(&self) -> bool {
        return (self.price_list.len() - 1) == self.current_price_index;
    }

    fn get_current_price(&self) -> i32 {
        return self.price_list[self.current_price_index];
    }
}

#[derive(Component)]
/// Marker struct for Text2ds that show the price
struct PriceText;

const BUY_BOOP_STARTING_ORBIT: f32 = 0.30;

const BUY_BOOP_MARGIN_TO_PLANET_WHEN_ZOOMED_OUT: f32 = 73.;

pub const BUY_BOOP_STARTING_HEIGHT: f32 =
    PLANET_FILE_RADIUS + (BUY_BOOP_MARGIN_TO_PLANET_WHEN_ZOOMED_OUT / ZOOMED_OUT_PLANET_SCALE);

pub fn buy_boops_and_hoops_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, advance_moon_btn_orbits);
}

/// Creates a moon button that buys boops
pub fn create_buy_boop_button(
    r#loop: Entity,
    boop_prices: [i32; 15],
    mut commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    create_buy_btn::<AddBoop>(
        r#loop,
        boop_prices.to_vec(),
        &"buy-boop-showcase.png",
        BUY_BOOP_STARTING_ORBIT,
        &mut commands,
        &asset_server,
        Transform {
            translation: Vec3::new(0., 60., 1.),
            ..default()
        },
    )
}

/// Creates a hoop button that buys hoops
pub fn create_buy_hoop_button(
    r#loop: Entity,
    planet: Planet,
    hoop_prices: [i32; 8],
    mut commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    create_buy_btn::<AddHoop>(
        r#loop,
        hoop_prices.to_vec(),
        &planet.get_hoop_showcase_path(),
        BUY_BOOP_STARTING_ORBIT + PI,
        &mut commands,
        &asset_server,
        Transform {
            translation: Vec3::new(-10., 60., 1.),
            rotation: Quat::from_rotation_z(0.6),
            ..default()
        },
    )
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

/// Creates a buy moon btn.
///
/// \param T The command that is triggered on Buy
/// \param showcase_path the path to the image on top of the moon
/// \param starting_loop_position see Orbit::current_loop_position
fn create_buy_btn<T: Command>(
    r#loop: Entity,
    prices: Vec<i32>,
    showcase_path: &str,
    starting_loop_position: f32,
    commands: &mut Commands,
    asset_server: &AssetServer,
    showcase_transform: Transform,
) -> Entity
where
    T: From<Entity>,
{
    let moon_img = asset_server.load("moon-btn.png");
    let showcase_img = asset_server.load(showcase_path);
    let loot_symbol_img = asset_server.load("loot-symbol.png");
    let spacey_font = asset_server.load("SpaceGrotesk-Light.ttf");

    let buy_btn = commands
        .spawn((
            Sprite::from_image(moon_img),
            Orbit {
                current_loop_position: starting_loop_position,
                starting_transform: Transform {
                    translation: Vec3::new(0., BUY_BOOP_STARTING_HEIGHT, 0.),
                    ..default()
                },
            },
            Pickable::default(),
        ))
        .id();

    let showcase = commands
        .spawn((
            Sprite::from_image(showcase_img),
            showcase_transform,
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    const LOOT_SYMBOL_SCALE: f32 = 0.05;
    let loot_symbol = commands
        .spawn((
            Sprite::from_image(loot_symbol_img),
            Transform {
                translation: Vec3::new(-25., 0., 1.),
                scale: Vec3::splat(LOOT_SYMBOL_SCALE),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    let text = commands
        .spawn((
            Text2d::new(i32_to_display_str(prices[0])),
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

    commands
        .entity(buy_btn)
        .add_children(&[showcase, loot_symbol, text])
        .insert(MoonBtn {
            price_list: prices,
            current_price_index: 0,
            text,
            r#loop,
        })
        .observe(buy_new_x_on_click::<T>);

    commands.entity(r#loop).add_child(buy_btn);

    buy_btn
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

    if moon_btn.reached_max_buy_amount() {
        unsuccessful_buy_animation_and_sound(moon_btn_e, &mut commands, &asset_server);
        return;
    }

    let enough_loot = **loot >= moon_btn.get_current_price();
    if enough_loot {
        **loot -= moon_btn.get_current_price();
        moon_btn.current_price_index += 1;

        commands.spawn((
            AudioPlayer::new(asset_server.load("successful-buy.ogg")),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        ));
        commands.queue(T::from(moon_btn.r#loop));

        if !moon_btn.reached_max_buy_amount() {
            let new_price = moon_btn.get_current_price();
            commands
                .entity(moon_btn.text)
                .insert(Text2d::new(i32_to_display_str(new_price)));
        } else {
            commands.entity(moon_btn.text).insert(Text2d::new("-"));
        }

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
        unsuccessful_buy_animation_and_sound(moon_btn_e, &mut commands, &asset_server);
    }

    fn unsuccessful_buy_animation_and_sound(
        moon_btn_e: Entity,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) {
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
            AudioPlayer::new(asset_server.load("unsuccessful-buy.ogg")),
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

/// Converts i32 to a string to be displayed on the moon btns for price. Only supports whole single digits and whole tens, hundreds, and thousands up to 99000. Will panic otherwise.
/// Uses h and k for abbreviations of 100 and 1000 respectively.
fn i32_to_display_str(num: i32) -> String {
    if num > 99000 {
        panic!("Tried to display too big of a number");
    }

    if num >= 1000 {
        if (num % 1000) != 0 {
            panic!("Not a whole number!");
        }
        return " ".to_string() + &(num / 1000).to_string() + "k";
    }

    if num >= 100 {
        if (num % 100) != 0 {
            panic!("Not a whole number!");
        }
        return " ".to_string() + &(num / 100).to_string() + "h";
    }

    if num >= 10 {
        if (num % 10) != 0 {
            panic!("Not a whole number!");
        }

        return " ".to_string() + &num.to_string();
    }

    num.to_string()
}
