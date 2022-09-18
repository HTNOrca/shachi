use std::ops::Range;

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
        movement::{BoidParams, FishNeighbouring, Movement, OrcaNeighbouring, Sight},
    },
    fish::Fish,
    names::*,
    orca::{Gender, Orca, Pod, PodPool, Type},
};

#[derive(Default)]
pub struct Simulation {
    pub time: f32,
    pub orca_count: usize,

    timer: Stopwatch,
}

pub struct RunSimEvent {
    pub enable_orca: bool,
    pub pod_count: usize,
    pub pod_size: Range<usize>,
    pub pod_size_min: usize,
    pub pod_size_max: usize,

    pub enable_fish: bool,
    pub fish_count: usize,

    pub orca_params: BoidParams,
    pub fish_params: BoidParams,
}

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Simulation::default())
            .add_event::<RunSimEvent>()
            .add_system(run_sim_orca)
            .add_system(run_sim_fish)
            .add_system(sim_time)
            .add_system(sim_count);
    }
}

fn run_sim_orca(
    mut cmd: Commands,
    query: Query<Entity, With<Orca>>,
    mut pod_pool: ResMut<PodPool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<RunSimEvent>,
) {
    use std::f32::consts::PI;

    for event in events.iter() {
        if !event.enable_orca {
            continue;
        }

        // cleanup previous simulation
        for entity in &query {
            cmd.entity(entity).despawn_recursive();
        }

        use rand::{seq::SliceRandom, thread_rng, Rng};

        cmd.insert_resource(Simulation::default());

        for pod_id in 0..event.pod_count {
            // create a new pod
            let pod_name = format!(
                "{} {}",
                POD_NAME_ADJ.choose(&mut thread_rng()).unwrap(),
                POD_NAME_NOUN.choose(&mut thread_rng()).unwrap()
            );
            let mut pod = Pod {
                name: pod_name,
                members: vec![],
            };
            let pod_color = Color::rgb(
                thread_rng().gen_range(0..100) as f32 / 100.,
                thread_rng().gen_range(0..100) as f32 / 100.,
                thread_rng().gen_range(0..100) as f32 / 100.,
            );
            let pod_size = thread_rng().gen_range(event.pod_size_min..event.pod_size_max);

            let pod_spawn_pos = Vec2::new(
                thread_rng().gen_range(-100..100) as f32,
                thread_rng().gen_range(-100..100) as f32,
            );
            let pod_type = match thread_rng().gen_range(0..=1) {
                0 => Type::Resident,
                1 => Type::Transient,
                _ => unreachable!(),
            };

            for j in 0..pod_size {
                let id = cmd.spawn().id();
                pod.members.push(id);

                let spawn_offset = Vec2::new(
                    thread_rng().gen_range(-100..100) as f32,
                    thread_rng().gen_range(-100..100) as f32,
                );

                let rand_angle = thread_rng().gen_range(0..(360 as i32)) as f32 * PI / 180.;
                let velocity = Mat2::from_angle(rand_angle) * Vec2::X * 10.;

                // independent params
                let gender = match thread_rng().gen_range(0..=1) {
                    0 => Gender::Male,
                    1 => Gender::Female,
                    _ => unreachable!(),
                };
                let age = thread_rng().gen_range(5..50);

                // dependent params
                let name = match gender {
                    Gender::Male => MALE_NAMES.choose(&mut thread_rng()).unwrap(),
                    Gender::Female => FEMALE_NAMES.choose(&mut thread_rng()).unwrap(),
                };
                let mass = thread_rng().gen_range(2000..3000) as f32;

                let id = cmd.spawn().id();

                cmd.entity(id)
                    .insert(Orca {
                        name: String::from(*name),
                        gender,
                        age,
                        mass,
                        orca_type: pod_type.clone(),
                        pod_id: Some(pod_id),
                    })
                    .insert(OrcaNeighbouring::default())
                    .insert(Hunger(thread_rng().gen_range(0.0f32..0.3f32)))
                    .insert(Sight {
                        view_range: event.orca_params.view_range,
                        view_angle: 90.,
                    })
                    .insert(Movement {
                        coherence: event.orca_params.coherence,
                        alignment: event.orca_params.alignment,
                        seperation: event.orca_params.seperation,
                        randomess: event.orca_params.randomness,
                        tracking: 10.,
                        wander_angle: 20,
                        target: None,
                        speed_scale: thread_rng().gen_range(90..110) as f32 / 10.,
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
                            .picker(FirstToScore { threshold: 0.5 })
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
                                outline_mode: StrokeMode::new(
                                    if gender.clone() == Gender::Male {
                                        Color::BLACK
                                    } else {
                                        Color::GRAY
                                    },
                                    0.1,
                                ),
                            },
                            Transform::from_scale(Vec3::splat(0.8 + age as f32 / 50.)),
                        ));
                    })
                    .insert_bundle(PickableBundle::default());
            }

            pod_pool.insert(pod_id as usize, pod);
        }
    }
}

fn run_sim_fish(
    mut cmd: Commands,
    query: Query<Entity, With<Fish>>,
    mut events: EventReader<RunSimEvent>,
) {
    use std::f32::consts::PI;

    use rand::{thread_rng, Rng};

    for event in events.iter() {
        if !event.enable_fish {
            continue;
        }

        for i in 0..=event.fish_count {
            let id = cmd.spawn().id();

            let spawn_pos = Vec2::new(
                thread_rng().gen_range(-100..100) as f32,
                thread_rng().gen_range(-100..100) as f32,
            );

            let rand_angle = thread_rng().gen_range(0..(360 as i32)) as f32 * PI / 180.;
            let velocity = Mat2::from_angle(rand_angle) * Vec2::X * 10.;

            cmd.entity(id)
                .insert(Fish)
                .insert(FishNeighbouring::default())
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    transform: Transform::from_translation(spawn_pos.extend(0.)),
                    ..default()
                })
                .insert(Sight {
                    view_range: event.fish_params.view_range,
                    view_angle: 90.,
                })
                .insert(Movement {
                    coherence: event.fish_params.coherence,
                    alignment: event.fish_params.alignment,
                    seperation: event.fish_params.seperation,
                    randomess: event.fish_params.randomness,
                    tracking: 10.,
                    wander_angle: 20,
                    target: None,
                    speed_scale: thread_rng().gen_range(90..110) as f32 / 10.,
                    ..default()
                })
                .insert(RigidBody {
                    max_velocity: Some(20.),
                    velocity,
                    mass: 1.,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(GeometryBuilder::build_as(
                        &RegularPolygon {
                            sides: 4,
                            ..default()
                        },
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::RED),
                            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
                        },
                        Transform::default(),
                    ));
                });
        }
    }
}

fn sim_time(time: Res<Time>, mut sim: ResMut<Simulation>) {
    sim.timer.tick(time.delta());
    sim.time = sim.timer.elapsed_secs();
}

fn sim_count(query: Query<&Orca>, mut sim: ResMut<Simulation>) {
    sim.orca_count = query.iter().len();
}
