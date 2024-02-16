use bevy::{
    input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent},
    prelude::*,
};

use crate::car::{Car, CarUpdateSet};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (gamepad_connections, gamepad_input.before(CarUpdateSet)),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct GamepadRes(Gamepad);

pub fn gamepad_connections(mut commands: Commands, mut gamepad_evr: EventReader<GamepadEvent>) {
    for event in gamepad_evr.read() {
        if let GamepadEvent::Connection(GamepadConnectionEvent {
            gamepad,
            connection,
        }) = event
        {
            match connection {
                GamepadConnection::Disconnected => commands.remove_resource::<GamepadRes>(),
                GamepadConnection::Connected(_) => {
                    println!("connect!");
                    commands.insert_resource(GamepadRes(*gamepad))
                }
            }
        }
    }
}

pub fn gamepad_input(
    gamepad_res: Option<Res<GamepadRes>>,
    axes: Res<Axis<GamepadAxis>>,
    button_axes: Res<Axis<GamepadButton>>,
    mut query: Query<&mut Car, With<Player>>,
) {
    let gamepad: Gamepad;
    match gamepad_res {
        None => return,
        Some(g) => gamepad = g.0,
    }
    let turn = axes
        .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
        .expect("gamepad error");
    let speed = button_axes
        .get(GamepadButton::new(
            gamepad,
            GamepadButtonType::RightTrigger2,
        ))
        .expect("gamepad error");
    let brake = button_axes
        .get(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
        .expect("gamepad error");
    for mut car in query.iter_mut() {
        car.power_target = speed;
        car.steering_target = turn;
        car.brake_target = brake;
    }
}
