mod hunger;
mod reproduction;
mod movement;

use bevy::prelude::*;
use big_brain::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin);
    }
}
