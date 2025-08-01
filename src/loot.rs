mod tweens;

use bevy::prelude::*;
use bevy_tweening::Animator;
use tweens::slide_in_from_right_tween;

/// main currency, used to buy more boops and hoops
#[derive(Resource, Deref, DerefMut)]
pub struct Loot(i32);

/// Marker struct for the Loot Display
#[derive(Component)]
struct LootDisplay;

/// Marker struct for the entity that contains the Text of the LootDisplay representing current loot
#[derive(Component)]
struct CurrentLootText;

/// The starting position left is off screen because it will slide in. See tweens::slide_in_from_right_tween
const STARTING_LEFT_POSITION: Val = Val::Px(200.);

pub fn loot_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_display)
        .add_systems(
            FixedUpdate,
            update_loot_display.run_if(resource_changed::<Loot>),
        )
        .insert_resource(Loot(0));
}

/// Wrap the system in a custom command for easier calling
pub struct ShowDisplay;
impl Command for ShowDisplay {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached(show_display);
    }
}

/// Make the loot counter visible by sliding it in from the right with a tween
fn show_display(loot_display: Single<Entity, With<LootDisplay>>, mut commands: Commands) {
    commands
        .entity(*loot_display)
        .insert(Animator::new(slide_in_from_right_tween()));
}

fn spawn_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    let loot_symbol = asset_server.load("loot-symbol.png");
    let spacey_font = asset_server.load("SpaceGrotesk-Light.ttf");

    let loot_display = commands
        .spawn((
            LootDisplay,
            Node {
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(1.)),
                margin: UiRect::left(Val::Auto)
                    .with_right(Val::Px(50.))
                    .with_top(Val::Px(30.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                left: STARTING_LEFT_POSITION, // For use with sliding animation
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.1)),
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
        ))
        .id();

    let loot_symbol = commands
        .spawn((
            ImageNode::new(loot_symbol),
            Node {
                width: Val::Px(30.),
                height: Val::Px(30.),
                margin: UiRect::left(Val::Px(10.)),
                ..default()
            },
        ))
        .id();

    let loot_text_container = commands
        .spawn(Node {
            width: Val::Px(80.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .id();

    commands.entity(loot_text_container).with_child((
        CurrentLootText,
        Text::default(),
        TextFont {
            font: spacey_font,
            font_size: 40.,
            ..default()
        },
        Node {
            margin: UiRect::left(Val::Px(-5.)),
            ..default()
        },
    ));

    commands
        .entity(loot_display)
        .add_children(&[loot_symbol, loot_text_container]);
}

/// Update the loot display
fn update_loot_display(loot: Res<Loot>, mut text: Single<&mut Text, With<CurrentLootText>>) {
    text.0 = loot.to_string();
}
