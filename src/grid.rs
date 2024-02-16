use bevy::{math::vec3, prelude::*};

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball = asset_server.load("Ball.png");
    for i in -100..=100 {
        for j in -100..=100 {
            commands.spawn(SpriteBundle {
                texture: ball.clone(),
                transform: Transform::from_translation(vec3(
                    i as f32 * 500.0,
                    j as f32 * 500.0,
                    -1.0,
                )),
                ..default()
            });
        }
    }
}
