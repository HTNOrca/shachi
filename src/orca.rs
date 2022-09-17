
use bevy::prelude::*;

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
}

pub struct OrcaBundle {

}
