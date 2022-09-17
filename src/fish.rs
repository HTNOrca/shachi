use bevy::prelude::*;
use bevy_bobs::physics_2d::RigidBody;

use crate::ai::movement::{Movement, Sight};

#[derive(Component)]
struct Fish;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(debug);
    }
}

fn debug(mut cmd: Commands) {
    use std::f32::consts::PI;

    use rand::{thread_rng, Rng};

    for i in 0..100 {
        let id = cmd.spawn().id();

        let spawn_pos = Vec2::new(
            thread_rng().gen_range(-100..100) as f32,
            thread_rng().gen_range(-100..100) as f32,
        );

        let rand_angle = thread_rng().gen_range(0..(360 as i32)) as f32 * PI / 180.;
        let velocity = Mat2::from_angle(rand_angle) * Vec2::X * 10.;

        cmd.entity(id).insert(Fish).insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_translation(spawn_pos.extend(0.)),
            ..default()
        });
    }
}
