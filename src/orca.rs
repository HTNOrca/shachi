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
        app.insert_resource(PodPool(HashMap::new()))
            .add_startup_system(debug);
    }
}

fn debug(
    mut cmd: Commands,
    mut pod_pool: ResMut<PodPool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use std::f32::consts::PI;

    use rand::{thread_rng, Rng};

    for pod_id in 0..10 {
        // create a new pod
        let mut pod = Pod::new();
        let pod_color = Color::rgb(
            thread_rng().gen_range(0..100) as f32 / 100.,
            thread_rng().gen_range(0..100) as f32 / 100.,
            thread_rng().gen_range(0..100) as f32 / 100.,
        );
        let pod_size = thread_rng().gen_range(15..30);

        let pod_spawn_pos = Vec2::new(
            thread_rng().gen_range(-100..100) as f32,
            thread_rng().gen_range(-100..100) as f32,
        );

        for j in 0..pod_size {
            let id = cmd.spawn().id();
            pod.members.push(id);

            let spawn_offset = Vec2::new(
                thread_rng().gen_range(-20..20) as f32,
                thread_rng().gen_range(-20..20) as f32,
            );

            let rand_angle = thread_rng().gen_range(0..(360 as i32)) as f32 * PI / 180.;
            let velocity = Mat2::from_angle(rand_angle) * Vec2::X * 10.;

            cmd.entity(id)
                .insert(Orca {
                    gender: Gender::Male,
                    age: 20,
                    mass: 3000.,
                    orca_type: Type::Resident,
                    pod_id: Some(pod_id),
                })
                .insert(Hunger::default())
                // .insert_bundle(GeometryBuilder::build_as(
                //     &RegularPolygon {
                //         sides: 3,
                //         ..default()
                //     },
                //     DrawMode::Outlined {
                //         fill_mode: FillMode::color(pod_color),
                //         outline_mode: StrokeMode::new(Color::BLACK, 0.1),
                //     },
                //     Transform::from_translation((pod_spawn_pos + spawn_offset).extend(0.)),
                // ))
                .insert(Sight::new(20., 90.))
                .insert(Movement {
                    coherence: 1.,
                    alignment: 1.,
                    seperation: 1.,
                    randomess: 5.0,
                    tracking: 0.,
                    wander_angle: 20,
                    target: None,
                    ..default()
                })
                .insert(RigidBody {
                    max_velocity: Some(20.),
                    velocity,
                    mass: 1.,
                    ..default()
                })
                .insert(
                    Thinker::build()
                        .picker(FirstToScore { threshold: 0.8 })
                        .when(Hungry, Hunt),
                )
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::new(3.))).into(),
                    transform: Transform::from_translation(
                        (pod_spawn_pos + spawn_offset).extend(0.),
                    ),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                })
                .insert_bundle(PickableBundle::default());
        }

        pod_pool.insert(pod_id as usize, pod);
    }
}
