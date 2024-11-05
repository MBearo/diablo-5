use crate::{
    components::{Actor, ActorType},
    util::{BOUNDARY_HEIGHT, BOUNDARY_WIDTH},
};
use bevy::prelude::*;

pub fn handle_movement(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Actor, &mut Transform)>,
) {
    let half_width = (BOUNDARY_WIDTH / 2) as f32;
    let half_height = (BOUNDARY_HEIGHT / 2) as f32;
    for (entity, mut actor, mut transform) in query.iter_mut() {
        let movement = actor.handle_movement(&keys, &time);
        let new_position = transform.translation + movement;

        match actor.actor_type {
            ActorType::Bullet(_) => {
                // 子弹出界就销毁
                if new_position.x.abs() > half_width || new_position.y.abs() > half_height {
                    commands.entity(entity).despawn();
                } else {
                    transform.translation = new_position;
                }
            }
            ActorType::Player | ActorType::Enemy => {
                // 玩家和敌人禁止出界
                transform.translation = Vec3::new(
                    new_position.x.clamp(-half_width, half_width),
                    new_position.y.clamp(-half_height, half_height),
                    new_position.z,
                );
            }
        }
    }
}
