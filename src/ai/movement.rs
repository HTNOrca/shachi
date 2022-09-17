use bevy::prelude::*;
use bevy_bobs::physics_2d::*;
use std::collections::HashMap;

use crate::orca::{PodPool, Orca};

#[derive(Component)]
pub struct Sight {
    pub view_angle: f32,
    pub view_range: f32,
    
    visible_pod_members: Vec<Entity>,
}

impl Sight {
    pub fn new(view_angle: f32, view_range: f32) -> Self {
        Self {
            view_angle,
            view_range,

            visible_pod_members: vec![],
        }
    }
}

/// ai with flocking behavior
#[derive(Component, Clone)]
pub struct Movement {
    /// weight for coherence
    pub coherence: f32,
    /// weight for alignment
    pub alignment: f32,
    /// weight for separation
    pub seperation: f32,
    /// weight for randomness
    pub randomess: f32,
    /// weight for tracking
    pub tracking: f32,
    /// range between 0..359
    pub wander_angle: u32,
    /// optional target to move towards
    pub target: Option<Vec2>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            coherence: 1.,
            alignment: 1.,
            seperation: 1.,
            randomess: 1.,
            tracking: 1.,
            wander_angle: 10,
            target: None,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(pod_member_sight)
            .add_system(boid_ai);        
    }
}

fn pod_member_sight(mut query: Query<(Entity, &Orca, &mut Transform, &mut Sight, &Movement)>, pod_pool: Res<PodPool>) {
    let mut updates: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for (self_entity, self_orca, self_trans, self_sight, self_movement) in query.iter() {
        // fetch all boids in viewing range
        let mut neighbours: Vec<Entity> = vec![];
        for (other_entity, other_orca, other_trans, other_sight, other_ai) in query.iter() {
            if self_entity == other_entity {
                continue;
            }
            if self_orca.pod_id.is_none() || other_orca.pod_id.is_none() || self_orca.pod_id != other_orca.pod_id {
                continue;
            }
            if self_trans.translation.distance(other_trans.translation) < self_sight.view_range {
                neighbours.push(other_entity);
            }
        }

        updates.insert(self_entity, neighbours);
    }

    for (e, n) in updates.iter() {
        let (_, _, _, mut sight, _) = query.get_mut(*e).unwrap();
        sight.visible_pod_members.clear();
        sight.visible_pod_members.extend(n);
    }
}

fn boid_ai(time: Res<Time>, mut query: Query<(Entity, &mut Transform, &Sight, &Movement, &mut RigidBody)>) {
    let mut force_updates: HashMap<Entity, Vec2> = HashMap::new();
    for (entity, trans, sight, movement, rb) in query.iter() {

        let neighbours = &sight.visible_pod_members;

        if neighbours.len() == 0 {
            continue;
        }

        let mut cur_force = force_updates
            .get(&entity)
            .unwrap_or(&Vec2::ZERO)
            .clone();

        // randomness force
        use rand::{thread_rng, Rng};
        use std::f32::consts::PI;

        if rb.velocity.length() != 0. {
            let rand: i32 = thread_rng().gen_range(0..(movement.wander_angle as i32));
            let angle_deviation = ((rand - 180) as f32) * PI / 180.;
            let forward = rb.velocity.angle_between(Vec2::X);
            let random_force = Mat2::from_angle(angle_deviation + forward) * Vec2::X;
            cur_force += random_force * movement.randomess;
        }

        // alignment (attempt to face same direction as neighbours)
        let avg_heading = query
            .iter_many(neighbours)
            .fold(Vec2::ZERO, |acc, (_, _, _, _, rb)| acc + rb.velocity)
            / neighbours.len() as f32;
        cur_force += avg_heading * movement.alignment;

        // cohesion
        let avg_position = query
            .iter_many(neighbours)
            .fold(Vec3::ZERO, |acc, (_, trans, _, _, _)| acc + trans.translation)
            / neighbours.len() as f32;
        cur_force += (avg_position - trans.translation).truncate() * movement.coherence;

        // separation
        let seperation_force = query.iter_many(neighbours).fold(Vec2::ZERO, |acc, (_, other_trans, _, _, _)| {
            let dist = trans.translation.distance(other_trans.translation);
            let dir = (trans.translation - other_trans.translation).truncate();
            // TODO careful for division by zero
            acc + dir / dist
        });
        cur_force += seperation_force * movement.seperation;

        /*
        // target
        if let Some(target) = self_ai.target {
            let target_force = target - self_trans.translation.truncate();
            cur_force += target_force * self_ai.tracking;
        }

        */
        force_updates.insert(entity, cur_force);
    }

    // update all the forces
    for (e, _, _, ai, mut rb) in query.iter_mut() {
        if let Some(force) = force_updates.get(&e) {
            rb.force += *force * time.delta().as_micros() as f32 / 1_000_000.0;
        }
    }
}
