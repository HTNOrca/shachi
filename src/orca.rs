
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
    gender: Gender,
    age: u32,
    weight: f32,
    orca_type: Type,
    speed: f32,
}

pub struct OrcaBundle {

}
