use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_bobs::physics_2d::*;

use crate::{orca::{Orca, Gender, Type}, ai::{hunger::Hunger, movement::{Sight, Movement}, AIPlugin}};

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
        .add_plugin(PhysicsPlugin);

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

fn debug(mut cmd: Commands) {
    use rand::{thread_rng, Rng};

    for i in 0..100 {

        let id = cmd.spawn().id();

        let spawn_pos = Vec2::new(thread_rng().gen_range(-100..100) as f32, thread_rng().gen_range(-100..100) as f32);

        cmd.entity(id)
            .insert(Orca {
                gender: Gender::Male,
                age: 20,
                mass: 3000.,
                orca_type: Type::Resident,
                pod_id: None
            })
            .insert(Hunger::default())
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
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
                ..default()
            });

    }
}
