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

pub struct SpawnOrcaEvent;
pub struct DespawnOrcaEvent(pub Entity);

pub type PodId = usize;

#[enum_string]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

#[enum_string]
#[derive(Clone, Copy)]
pub enum Type {
    Resident,
    Transient,
}

#[derive(Component)]
pub struct Orca {
    pub name: String,
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
    pub name: String,
    pub members: Vec<Entity>,
}

#[derive(Deref, DerefMut)]
pub struct PodPool(pub HashMap<PodId, Pod>);

pub struct OrcaPlugin;

impl Plugin for OrcaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PodPool(HashMap::new()))
            .add_event::<SpawnOrcaEvent>()
            .add_event::<DespawnOrcaEvent>()
            .add_system(despawn);
    }
}

fn despawn(mut cmd: Commands, mut events: EventReader<DespawnOrcaEvent>) {
    for DespawnOrcaEvent(entity) in events.iter() {
        println!("orca has passed away {:?}", entity);
        cmd.entity(*entity).despawn_recursive();
    }
}
