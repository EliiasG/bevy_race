use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use camera_controller::{CameraControllerPlugin, PlayerCamera};
use car::{Car, Wheel};
use grid::GridPlugin;
use player_controller::{Player, PlayerControllerPlugin};
use rigidbody::Rigidbody;

use crate::{car::CarPlugin, rigidbody::RigidbodyPlugin};

pub mod camera_controller;
pub mod car;
pub mod grid;
pub mod player_controller;
pub mod rigidbody;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ALICE_BLUE))
        .add_plugins((
            DefaultPlugins,
            RigidbodyPlugin,
            CarPlugin,
            PlayerControllerPlugin,
            CameraControllerPlugin,
            GridPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, testing)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wheels = [
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("Wheel.png"),
                    transform: Transform {
                        translation: vec3(14.0, 19.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Wheel {
                    steering: 1.0,
                    ..default()
                },
            ))
            .id(),
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("Wheel.png"),
                    transform: Transform {
                        translation: vec3(-14.0, 19.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Wheel {
                    steering: 1.0,
                    ..default()
                },
            ))
            .id(),
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("Wheel.png"),
                    transform: Transform {
                        translation: vec3(14.0, -23.5, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Wheel {
                    steering: 0.0,
                    drive: true,
                    brake: true,
                    ..default()
                },
            ))
            .id(),
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("Wheel.png"),
                    transform: Transform {
                        translation: vec3(-14.0, -23.5, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Wheel {
                    steering: 0.0,
                    drive: true,
                    brake: true,
                    ..default()
                },
            ))
            .id(),
    ];
    let zoom = 2.0;
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(vec3(zoom, zoom, zoom)),
            ..default()
        },
        PlayerCamera,
    ));
    let mut rb = Rigidbody::default();
    rb.velocity = vec2(0.0, 0.0);
    rb.linear_mass = 5.0;
    rb.angular_mass = rb.linear_mass * 1000.0;
    let steer = (20.0 as f32).to_radians();
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("Car.png"),
                ..default()
            },
            rb,
            Car {
                steering_angle_max: steer,
                steering_speed: steer * 5.0,
                steering_target: 0.0,
                speed_max: 15000.0,
                power: 150.0,
                brake_power: 4000.0,
                ..default()
            },
            Player,
        ))
        .insert_children(0, &wheels);
}

fn testing(query: Query<&Transform>) {
    for transform in query.iter() {
        debug!("{}", transform.translation);
    }
}
