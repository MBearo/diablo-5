use crate::components::{Actor, Bullet, Health, Position};
use ggez::glam::Vec2;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Actor>,
    );

    fn run(&mut self, (entities, positions, bullets, mut healths, actors): Self::SystemData) {
        // 检查子弹碰撞
        for (bullet_entity, bullet_pos, bullet) in (&entities, &positions, &bullets).join() {
            for (target_entity, target_pos, target_health, target_actor) in
                (&entities, &positions, &mut healths, &actors).join()
            {
                // 跳过同类型的实体
                if target_actor.actor_type == bullet.owner_type {
                    continue;
                }

                // 简单的碰撞检测
                let distance =
                    Vec2::new(target_pos.x - bullet_pos.x, target_pos.y - bullet_pos.y).length();

                if distance < 20.0 {
                    // 碰撞半径
                    // 扣血
                    target_health.current -= bullet.damage;
                    // 删除子弹
                    entities
                        .delete(bullet_entity)
                        .expect("Failed to delete bullet");

                    // 如果目标血量为0，删除目标
                    if target_health.current <= 0.0 {
                        entities
                            .delete(target_entity)
                            .expect("Failed to delete entity");
                    }
                }
            }
        }
    }
}
