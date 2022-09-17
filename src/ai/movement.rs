use bevy::prelude::*;
use bevy_bobs::physics_2d::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Sight {
    pub view_angle: f32,
    pub view_range: f32,
}

impl Default for Sight {
    fn default() -> Self {
        Self {
            view_angle: 90.,
            view_range: 100.,
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
            coherence: 0.,
            alignment: 0.,
            seperation: 0.,
            randomess: 0.,
            tracking: 0.,
            wander_angle: 0,
            target: None,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(boid_ai_system);        
    }
}

pub fn boid_ai_system(mut query: Query<(Entity, &mut Transform, &Sight, &Movement, &mut RigidBody)>) {
    let mut force_updates: HashMap<Entity, Vec2> = HashMap::new();
    for (self_entity, self_trans, self_sight, self_ai, self_rb) in query.iter() {
        // fetch all boids in viewing range
        let mut neighbours: Vec<(Transform, Movement, RigidBody)> = vec![];
        for (other_entity, other_trans, other_sight, other_ai, other_rb) in query.iter() {
            if self_entity == other_entity {
                continue;
            }
            if self_trans.translation.distance(other_trans.translation) < self_sight.view_range {
                neighbours.push((other_trans.clone(), other_ai.clone(), other_rb.clone()));
            }
        }

        if neighbours.len() == 0 {
            continue;
        }

        let mut cur_force = force_updates
            .get(&self_entity)
            .unwrap_or(&Vec2::ZERO)
            .clone();

        // randomness force
        use rand::{thread_rng, Rng};
        use std::f32::consts::PI;

        let rand: i32 = thread_rng().gen_range(0..(self_ai.wander_angle as i32));
        let angle_deviation = ((rand - 180) as f32) * PI / 180.;
        let forward = self_rb.velocity.angle_between(Vec2::X);
        let random_force =
            (Mat2::from_angle(angle_deviation + forward) * Vec2::X) * self_ai.randomess;
        cur_force += random_force;

        // alignment (attempt to face same direction as neighbours)
        let avg_heading = neighbours
            .iter()
            .fold(Vec2::ZERO, |acc, (_, _, rb)| acc + rb.velocity)
            / neighbours.len() as f32;
        cur_force += avg_heading * self_ai.alignment + cur_force;

        // cohesion
        let avg_position = neighbours
            .iter()
            .fold(Vec3::ZERO, |acc, (trans, _, _)| acc + trans.translation)
            / neighbours.len() as f32;
        cur_force += (avg_position - self_trans.translation).truncate() * self_ai.coherence;

        // separation
        let seperation_force = neighbours.iter().fold(Vec2::ZERO, |acc, (trans, _, _)| {
            let dist = trans.translation.distance(self_trans.translation);
            let dir = (self_trans.translation - trans.translation).truncate();
            acc + dir / dist
        });
        cur_force += seperation_force * self_ai.seperation;

        // target
        if let Some(target) = self_ai.target {
            let target_force = target - self_trans.translation.truncate();
            cur_force += target_force * self_ai.tracking;
        }

        force_updates.insert(self_entity, cur_force);
    }

    // update all the forces
    for (e, _, _, ai, mut rb) in query.iter_mut() {
        if let Some(force) = force_updates.get(&e) {
            rb.force += *force * 5.;
        }
    }
}
