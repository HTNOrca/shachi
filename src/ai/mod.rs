pub mod hunger;
pub mod reproduction;
pub mod movement;

use bevy::prelude::*;
use big_brain::prelude::*;

use self::{hunger::HungerPlugin, movement::MovementPlugin};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_plugin(HungerPlugin)
            .add_plugin(MovementPlugin);
    }
}
