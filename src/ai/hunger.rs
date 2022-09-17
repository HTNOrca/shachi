use bevy::prelude::*;

const HUNGER_RATE: f32 = 0.001;

#[derive(Component)]
pub struct Hunger {
    /// Hunger percentage
    pub hunger: f32,
}

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(passive_hunger_system);
    }
}

fn passive_hunger_system(time: Res<Time>, mut hunger: Query<&mut Hunger>) {
    for mut hunger in hunger.iter_mut() {
        hunger.hunger += HUNGER_RATE * time.delta().as_micros() as f32 / 1_000_000.0;
        hunger.hunger = hunger.hunger.clamp(0., 1.);
    }
}

