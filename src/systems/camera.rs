use crate::components::{Actor};
use bevy::prelude::*;
use crate::util::consts::DEFAULT_ACTOR_SPEED;

pub fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Actor>,
) {
    // 获取 玩家 的速度
    let player_speed = if let Ok(player) = player_query.get_single() {
        player.speed
    } else {
        DEFAULT_ACTOR_SPEED
    };

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        camera_transform.translation += direction * player_speed * time.delta_seconds();
    }
}
