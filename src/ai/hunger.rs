use bevy::prelude::*;
use big_brain::prelude::*;

use super::movement::{Movement, OrcaNeighbouring, Sight};
use crate::orca::{DespawnOrcaEvent, Orca};

const HUNGER_RATE: f32 = 0.001;

#[derive(Component, Default, Deref)]
pub struct Hunger(pub f32);

impl Hunger {
    fn eat(&mut self, amount: f32) {
        self.0 = (self.0 + amount).clamp(0., 1.);
    }
}

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(passive_hunger_system)
            .add_system(passive_hunger_system);

        app.add_system(hungry_scorer).add_system(hunt_action);
    }
}

fn passive_hunger_system(
    time: Res<Time>,
    mut hunger: Query<(Entity, &mut Hunger)>,
    mut writer: EventWriter<DespawnOrcaEvent>,
) {
    for (entity, mut hunger) in hunger.iter_mut() {
        hunger.0 += HUNGER_RATE * time.delta().as_micros() as f32 / 1_000_000.0;
        if hunger.0 >= 1. {
            writer.send(DespawnOrcaEvent(entity));
        }
        hunger.0 = hunger.0.clamp(0., 1.);
    }
}

#[derive(Clone, Component, Debug)]
pub struct Hungry;

fn hungry_scorer(hungers: Query<&Hunger>, mut query: Query<(&Actor, &mut Score), With<Hungry>>) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(hunger) = hungers.get(*actor) {
            score.set(hunger.0);
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct Hunt;

fn hunt_action(
    mut cmd: Commands,
    mut actor_query: Query<(&Transform, &mut Hunger, &OrcaNeighbouring, &mut Movement), With<Orca>>,
    mut prey_query: Query<(&Transform), Without<Orca>>,
    mut query: Query<(&Actor, &mut ActionState, &Hunt)>,
) {
    for (Actor(actor), mut state, hunt) in query.iter_mut() {
        if let Ok((trans, mut hunger, neighbours, mut movement)) = actor_query.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    // search for prey (patrol routes?)

                    // check if prey was found
                    if neighbours.prey.len() > 0 {
                        // TODO better way of choosing a fish to hunt
                        let target = neighbours.prey[0];
                        movement.target = Some(target);

                        *state = ActionState::Executing
                    }
                },
                ActionState::Executing => {
                    // if close enough to prey, eat it
                    const EAT_RANGE: f32 = 10.;
                    const GIVE_UP_RANGE: f32 = 300.;

                    if movement.target.is_none() {
                        *state = ActionState::Cancelled;
                        return;
                    }

                    if let Ok(prey_trans) = prey_query.get(movement.target.unwrap()) {
                        // Eat the prey
                        if trans.translation.distance(prey_trans.translation) < EAT_RANGE {
                            cmd.entity(movement.target.unwrap()).despawn_recursive();

                            hunger.eat(0.01);
                            movement.target = None;
                            *state = ActionState::Success;
                        }

                        // Give up persuing prey
                        if trans.translation.distance(prey_trans.translation) > GIVE_UP_RANGE {
                            *state = ActionState::Cancelled;
                        }
                    } else {
                        *state = ActionState::Cancelled;
                        return;
                    }
                },
                _ => {},
            }
        }
    }
}
