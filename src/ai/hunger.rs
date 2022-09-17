use bevy::prelude::*;

const HUNGER_RATE: f32 = 0.001;

#[derive(Component, Default, Deref)]
pub struct Hunger(pub f32);

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(passive_hunger_system);
    }
}

fn passive_hunger_system(time: Res<Time>, mut hunger: Query<&mut Hunger>) {
    for mut hunger in hunger.iter_mut() {
        hunger.0 += HUNGER_RATE * time.delta().as_micros() as f32 / 1_000_000.0;
        hunger.0 = hunger.0.clamp(0., 1.);
    }
}

