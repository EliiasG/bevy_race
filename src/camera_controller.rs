use bevy::prelude::*;

use crate::player_controller::Player;

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cameras);
    }
}

#[derive(Component)]
pub struct PlayerCamera;

fn update_cameras(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };
    for mut camera_transform in camera_query.iter_mut() {
        let camera_pos = camera_transform.translation.xy();
        let player_pos = player_transform.translation.xy();
        let diff = player_pos - camera_pos;
        camera_transform.translation += (diff * time.delta_seconds()).extend(0.0);
    }
}
