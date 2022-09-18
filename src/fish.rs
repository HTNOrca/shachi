use bevy::prelude::*;
use bevy_bobs::physics_2d::RigidBody;

use crate::ai::movement::{Movement, Sight};

#[derive(Component)]
pub struct Fish;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {}
}
