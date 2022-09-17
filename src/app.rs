use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_bobs::physics_2d::*;

use crate::{orca::{Orca, Gender, Type, OrcaPlugin, Pod, PodPool}, ai::{hunger::Hunger, movement::{Sight, Movement}, AIPlugin}};

pub fn app() {
    let mut window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "sakamata".into(),
        ..default()
    };

    window_descriptor.width = 800.;
    window_descriptor.height = 600.;

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(window_descriptor);

    app.add_plugins(DefaultPlugins)
        .add_plugin(AIPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(OrcaPlugin);

    app.add_startup_system(spawn_camera)
        .add_startup_system(debug);

    app.run();

}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        ..default()
    });
}

fn debug(mut cmd: Commands, mut pod_pool: ResMut<PodPool>) {

    use rand::{thread_rng, Rng};
    use std::f32::consts::PI;

    for pod_id in 0..10 {

        // create a new pod
        let mut pod = Pod::new();
        let pod_color = Color::rgb(
            thread_rng().gen_range(0..100) as f32 / 100.,
            thread_rng().gen_range(0..100) as f32 / 100.,
            thread_rng().gen_range(0..100) as f32 / 100.
        );
        let pod_size = thread_rng().gen_range(15..30);

        for j in 0..pod_size {

            let id = cmd.spawn().id();
            pod.members.push(id);

            let spawn_pos = Vec2::new(thread_rng().gen_range(-100..100) as f32, thread_rng().gen_range(-100..100) as f32);

            let rand_angle = thread_rng().gen_range(0..(360 as i32)) as f32 * PI / 180.;
            let velocity = Mat2::from_angle(rand_angle) * Vec2::X;

            cmd.entity(id)
                .insert(Orca {
                    gender: Gender::Male,
                    age: 20,
                    mass: 3000.,
                    orca_type: Type::Resident,
                    pod_id: Some(pod_id),
                })
                .insert(Hunger::default())
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: pod_color,
                        ..default()
                    },
                    transform: Transform::from_translation(spawn_pos.extend(0.)),
                    ..default()
                })
                .insert(Sight::new(50., 90.))
                .insert(Movement {
                    coherence: 1.,
                    alignment: 1.,
                    seperation: 1.,
                    randomess: 1.0,
                    tracking: 0.,
                    wander_angle: 10,
                    target: None,
                })
                .insert(RigidBody {
                    max_velocity: Some(10.),
                    velocity,
                    ..default()
                });

        }

        pod_pool.insert(pod_id as usize, pod);
    }
}
