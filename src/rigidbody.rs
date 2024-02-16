use std::f32::consts;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

const MARGIN: f32 = 0.0001;

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum RigidbodyUpdateSet {
    ApplyForces,
    UpdateTransform,
}

pub struct RigidbodyPlugin;

impl Plugin for RigidbodyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Update,
            (
                RigidbodyUpdateSet::ApplyForces,
                RigidbodyUpdateSet::UpdateTransform,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                apply_forces.in_set(RigidbodyUpdateSet::ApplyForces),
                update_transform.in_set(RigidbodyUpdateSet::UpdateTransform),
            ),
        );
    }
}

#[derive(Component, Default)]
pub struct Rigidbody {
    pub linear_mass: f32,
    pub angular_mass: f32,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    velocity_delta: Vec2,
    angular_velocity_delta: f32,
}

impl Rigidbody {
    pub fn add_force(&mut self, force: Vec2, pos: Vec2) {
        if force.abs_diff_eq(Vec2::ZERO, MARGIN) {
            return;
        }
        self.velocity_delta += force / self.linear_mass;
        // point on force line closest
        let p = -pos.project_onto(force) + pos;
        if p.abs_diff_eq(Vec2::ZERO, MARGIN) {
            return;
        }
        let len_sq = p.length_squared();
        let rot_speed = len_sq / self.angular_mass * force.length();
        if pos.perp_dot(force) > 0.0 {
            self.angular_velocity_delta += rot_speed;
        } else {
            self.angular_velocity_delta -= rot_speed;
        }
    }

    pub fn get_velocity_at(&self, pos: Vec2) -> Vec2 {
        let r = pos.yx() * 0.5 * vec2(1.0, -1.0) * consts::TAU * self.angular_velocity;
        return self.velocity - r;
    }
}

fn apply_forces(mut query: Query<&mut Rigidbody>) {
    for mut rb in query.iter_mut() {
        let velocity_delta = rb.velocity_delta;
        let angular_delta = rb.angular_velocity_delta;

        rb.velocity += velocity_delta;
        rb.angular_velocity += angular_delta;

        rb.velocity_delta = Vec2::ZERO;
        rb.angular_velocity_delta = 0.0;
    }
}

fn update_transform(time: Res<Time>, mut query: Query<(&mut Transform, &Rigidbody)>) {
    let delta = time.delta_seconds();
    for (mut transform, rb) in query.iter_mut() {
        let scale = transform.scale;
        transform.translation += vec3(rb.velocity.x, rb.velocity.y, 0.0) * delta * scale;
        transform.rotate_z(rb.angular_velocity * delta);
    }
}
