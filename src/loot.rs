use bevy::prelude::*;

/// main currency, used to buy more boops and hoops
#[derive(Resource, Deref, DerefMut)]
pub struct Loot(i32);

pub fn loot_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        update_loot_display.run_if(resource_changed::<Loot>),
    )
    .insert_resource(Loot(0));
}

/// Show the the loot counter, with bevy easings so it doesn't abruptly show
pub fn show() {}

/// Update the loot display
pub fn update_loot_display(loot: Res<Loot>) {
    dbg!(**loot);
}
