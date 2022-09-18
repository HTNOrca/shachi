use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::Stopwatch};
use bevy_bobs::physics_2d::RigidBody;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::prelude::*;
use big_brain::prelude::*;
use iyes_loopless::prelude::*;
use pino_utils::enum_string;

use crate::{
    ai::{
        hunger::{Hunger, Hungry, Hunt},
        movement::{Movement, Sight},
    },
    orca::{Gender, Orca, Pod, PodPool, Type},
};

#[derive(Default)]
pub struct Simulation {
    pub time: f32,
    pub orca_count: usize,

    timer: Stopwatch,
}

pub struct RunSimEvent;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Simulation::default())
            .add_event::<RunSimEvent>()
            .add_system(run_sim.run_on_event::<RunSimEvent>())
            .add_system(sim_time)
            .add_system(sim_count);

        app.add_startup_system(run_sim);
    }
}

fn run_sim(
    mut cmd: Commands,
    mut pod_pool: ResMut<PodPool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use std::f32::consts::PI;

    use rand::{thread_rng, Rng};

    cmd.insert_resource(Simulation::default());

    for pod_id in 0..20 {
        // create a new pod
        let mut pod = Pod::new();
        let pod_color = Color::rgb(
            thread_rng().gen_range(0..100) as f32 / 100.,
            thread_rng().gen_range(0..100) as f32 / 100.,
            thread_rng().gen_range(0..100) as f32 / 100.,
        );
        let pod_size = thread_rng().gen_range(1..6);

        let pod_spawn_pos = Vec2::new(
            thread_rng().gen_range(-100..100) as f32,
            thread_rng().gen_range(-100..100) as f32,
        );

        for j in 0..pod_size {
            let id = cmd.spawn().id();
            pod.members.push(id);

            let spawn_offset = Vec2::new(
                thread_rng().gen_range(-100..100) as f32,
                thread_rng().gen_range(-100..100) as f32,
            );

            let rand_angle = thread_rng().gen_range(0..(360 as i32)) as f32 * PI / 180.;
            let velocity = Mat2::from_angle(rand_angle) * Vec2::X * 10.;

            let mass = thread_rng().gen_range(2000..3000) as f32;

            cmd.entity(id)
                .insert(Orca {
                    gender: Gender::Male,
                    age: 20,
                    mass,
                    orca_type: Type::Resident,
                    pod_id: Some(pod_id),
                })
                .insert(Hunger(0.99))
                .insert(Sight::new(20., 90.))
                .insert(Movement {
                    coherence: 1.,
                    alignment: 1.,
                    seperation: 1.,
                    randomess: 5.,
                    tracking: 10.,
                    wander_angle: 20,
                    target: None,
                    speed_scale: thread_rng().gen_range(90..110) as f32 / 10.,
                    ..default()
                })
                .insert(RigidBody {
                    max_velocity: Some(20.),
                    velocity,
                    mass,
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
                    material: materials.add(ColorMaterial::from(Color::NONE)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(GeometryBuilder::build_as(
                        &RegularPolygon {
                            sides: 3,
                            ..default()
                        },
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(pod_color),
                            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
                        },
                        Transform::default(),
                    ));
                })
                .insert_bundle(PickableBundle::default());
        }

        pod_pool.insert(pod_id as usize, pod);
    }
}

fn sim_time(time: Res<Time>, mut sim: ResMut<Simulation>) {
    sim.timer.tick(time.delta());
    sim.time = sim.timer.elapsed_secs();
}

fn sim_count(query: Query<&Orca>, mut sim: ResMut<Simulation>) {
    sim.orca_count = query.iter().len();
}

// pub fn cleanup(query: ) {

// }
