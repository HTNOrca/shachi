use std::collections::HashMap;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_bobs::physics_2d::RigidBody;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::prelude::*;
use big_brain::prelude::*;
use pino_utils::enum_string;

use crate::ai::{
    hunger::{Hunger, Hungry, Hunt},
    movement::{Movement, Sight},
};

pub type PodId = usize;

#[enum_string]
pub enum Gender {
    Male,
    Female,
}

#[enum_string]
pub enum Type {
    Resident,
    Transient,
}

#[derive(Component)]
pub struct Orca {
    pub gender: Gender,
    /// age in years
    pub age: u32,
    /// mass in kg
    pub mass: f32,
    pub orca_type: Type,
    pub pod_id: Option<PodId>,
}

pub struct OrcaBundle {}

pub struct Pod {
    pub members: Vec<Entity>,
}

impl Pod {
    pub fn new() -> Self {
        Self { members: vec![] }
    }
}

#[derive(Deref, DerefMut)]
pub struct PodPool(pub HashMap<PodId, Pod>);

pub struct OrcaPlugin;

impl Plugin for OrcaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PodPool(HashMap::new()));
    }
}
