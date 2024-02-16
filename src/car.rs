use bevy::{math::vec2, prelude::*};

use crate::rigidbody::{Rigidbody, RigidbodyUpdateSet};

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub struct CarUpdateSet;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(Update, CarUpdateSet.before(RigidbodyUpdateSet::ApplyForces))
            .add_systems(
                Update,
                (apply_wheel_forces, update_wheels)
                    .chain()
                    .in_set(CarUpdateSet),
            );
    }
}

#[derive(Component, Default)]
pub struct Car {
    // target steering amount, must be in range [-1, 1] (inclusive)
    pub steering_target: f32,
    // same as steering but for power
    pub power_target: f32,
    pub brake_target: f32,
    pub steering_angle_max: f32,
    pub steering_speed: f32,
    pub speed_max: f32,
    pub power: f32,
    pub brake_power: f32,
}

#[derive(Component, Default)]
pub struct Wheel {
    pub drive: bool,
    pub speed: f32,
    pub brake: bool,
    // steering multiplyer, can be negative for wheels in the back, or 0 for no steering
    pub steering: f32,
}

fn grip(velocity: f32) -> f32 {
    /*
    let cutoff = 60.0;
    let exponent = 1.5;
    return (1.0 - (velocity / cutoff).powf(exponent)).max(0.05);
     */
    /*
    if velocity == 0.0 {
        return 1.0;
    }
    let base = 10.0;
    return (base / velocity.powf(0.75)).clamp(0.0, 1.0);
     */

    if velocity < 100.0 {
        1.0
    } else {
        0.1
    }
}

fn apply_wheel_forces(
    time: Res<Time>,
    mut wheel_query: Query<(&GlobalTransform, &Parent, &mut Wheel)>,
    mut car_query: Query<(&Transform, &mut Rigidbody)>,
) {
    for (transform, parent, mut wheel) in wheel_query.iter_mut() {
        let (car_transform, mut car_rb) =
            car_query.get_mut(parent.get()).expect("wheel not in car");
        let pos = (transform.translation() - car_transform.translation).xy();
        let car_vel = car_rb.get_velocity_at(pos);
        let relative_vel = car_vel - transform.up().xy() * wheel.speed;
        //let vel = car_rb.velocity;

        let grip_mul = grip(relative_vel.length());
        let force = -relative_vel * time.delta_seconds() * grip_mul * 5.0;
        car_rb.add_force(force, pos);
        let proj = force.project_onto(transform.up().xy());
        let forward = proj.length();
        wheel.speed -= if (transform.up().xy() * forward).abs_diff_eq(proj, 0.01) {
            forward
        } else {
            -forward
        };
        println!("{}", relative_vel.length())

        /*
        let force_magnitude = (wheel.speed - forward) * time.delta_seconds() * grip_mul * 3.0;
        car_rb.add_force(transform.up().xy() * force_magnitude, pos);
        wheel.speed -= force_magnitude;
        let slide_vel = vel.project_onto(transform.right().xy());
        println!("{}", vel.length());
        //let slide_grip = grip(slide_vel.length());
        car_rb.add_force(slide_vel * -5.0 * time.delta_seconds() * grip_mul, pos)
        */
    }
}

fn update_wheels(
    time: Res<Time>,
    mut wheel_query: Query<(&mut Transform, &Parent, &mut Wheel)>,
    car_query: Query<&Car>,
) {
    for (mut transform, parent, mut wheel) in wheel_query.iter_mut() {
        let car = car_query.get(parent.get()).expect("wheel not in car");

        let angle = transform.rotation.to_euler(EulerRot::ZYX).0;
        let max_angle = car.steering_angle_max * wheel.steering;
        let speed = car.steering_speed * wheel.steering.abs() * time.delta_seconds();
        let new_angle = angle + (-max_angle * car.steering_target - angle).clamp(-speed, speed);
        transform.rotate_z(new_angle - angle);
        if wheel.drive {
            wheel.speed = (wheel.speed + car.power * time.delta_seconds() * car.power_target)
                .min(car.speed_max);
        }
        if wheel.brake {
            let brake_power = car.brake_power * car.brake_target * time.delta_seconds();
            wheel.speed -= brake_power * wheel.speed.signum();
        }
    }
}
