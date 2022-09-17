
use bevy::prelude::*;
use std::collections::HashMap;

pub type PodId = usize;

pub enum Gender {
    Male,
    Female
}

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

pub struct OrcaBundle {

}

pub struct Pod {
    pub members: Vec<Entity>,
}

impl Pod {
    pub fn new() -> Self {
        Self {
            members: vec![]
        }
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
