use bevy::prelude::*;
use big_brain::prelude::*;

const HUNGER_RATE: f32 = 0.001;

#[derive(Component, Default, Deref)]
pub struct Hunger(pub f32);

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(passive_hunger_system)
            .add_system(passive_hunger_system);

        app.add_system(hungry_scorer)
            .add_system(hunt_action);
    }
}

fn passive_hunger_system(time: Res<Time>, mut hunger: Query<&mut Hunger>) {
    for mut hunger in hunger.iter_mut() {
        hunger.0 += HUNGER_RATE * time.delta().as_micros() as f32 / 1_000_000.0;
        hunger.0 = hunger.0.clamp(0., 1.);
    }
}

#[derive(Clone, Component, Debug)]
pub struct Hungry;

fn hungry_scorer(
    hungers: Query<&Hunger>,
    mut query: Query<(&Actor, &mut Score), With<Hungry>>
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(hunger) = hungers.get(*actor) {
            score.set(hunger.0);
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct Hunt;

fn hunt_action(
    mut hungers: Query<&mut Hunger>,
    mut query: Query<(&Actor, &mut ActionState, &Hunt)>
) {
    for (Actor(actor), mut state, hunt) in query.iter_mut() {
        if let Ok(mut hunger) = hungers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // search for prey
                    *state = ActionState::Executing
                }
                ActionState::Executing => {

                }
                _ => {}
            }
        }
    }
}
