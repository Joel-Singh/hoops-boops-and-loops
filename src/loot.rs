use bevy::prelude::*;

/// main currency, used to buy more boops and hoops
#[derive(Resource, Deref, DerefMut)]
pub struct Loot(i32);

/// Marker struct for the Loot Display Text
#[derive(Component)]
struct LootDisplay;

pub fn loot_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_display)
        .add_systems(
            FixedUpdate,
            update_loot_display.run_if(resource_changed::<Loot>),
        )
        .insert_resource(Loot(0));
}

/// Show the the loot counter, with bevy easings so it doesn't abruptly show
pub fn show() {}

pub fn spawn_display(mut commands: Commands) {
    commands.spawn((LootDisplay, Text::new("0")));
}

/// Update the loot display
fn update_loot_display(loot: Res<Loot>, mut text: Single<&mut Text, With<LootDisplay>>) {
    text.0 = loot.to_string();
}
