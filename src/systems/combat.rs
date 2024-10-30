use ggez::glam::Vec2;
use specs::{Builder, Entities, Join, LazyUpdate, ReadStorage, System, Write, WriteStorage};

use crate::components::{Actor, ActorType, Bullet, Combat, Movable, Position, Renderable};

pub struct CombatSystem {
    pub delta_time: f32,
}

impl<'a> System<'a> for CombatSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Combat>,
        Write<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, positions, actors, mut combats, lazy): Self::SystemData) {
        // 遍历所有具有位置和战斗能力的实体
        for (entity, pos, actor, combat) in (&entities, &positions, &actors, &mut combats).join() {
            if actor.actor_type != ActorType::Player {
                continue;
            }
            // 更新攻击计时器
            combat.attack_timer -= self.delta_time;

            if combat.attack_timer <= 0.0 {
                // 寻找最近的目标
                let mut nearest_target = None;
                let mut min_distance = f32::MAX;

                for (target_pos, target_actor, _) in (&positions, &actors, &entities).join() {
                    // 跳过同类型的实体
                    if target_actor.actor_type == actor.actor_type {
                        continue;
                    }

                    let distance = Vec2::new(target_pos.x - pos.x, target_pos.y - pos.y).length();
                    
                    if distance < min_distance {
                        min_distance = distance;
                        nearest_target = Some((target_pos.x, target_pos.y));
                    }
                }
               
                // 如果找到目标，发射子弹
                if let Some((target_x, target_y)) = nearest_target {
                    println!("nearest_target {:?}", nearest_target);
                    let direction = Vec2::new(target_x - pos.x, target_y - pos.y).normalize();
                    let bullet_speed = 300.0;

                    println!(
                        "Creating bullet at ({}, {}) with direction: ({}, {})",
                        pos.x, pos.y, direction.x, direction.y
                    );
                    lazy.create_entity(&entities)
                        .with(Position {
                            x: pos.x,
                            y: pos.y,
                            z: 10,
                        })
                        .with(Renderable {
                            path: "/images/box_spot.png".to_string(),
                        })
                        .with(Movable {
                            velocity: direction * bullet_speed,
                        })
                        .with(Bullet {
                            damage: combat.attack_damage,
                            owner_type: actor.actor_type,
                            // speed: bullet_speed,
                        })
                        .build();

                    // 重置攻击计时器
                    combat.attack_timer = combat.attack_cooldown;
                }
            }
        }
    }
}
