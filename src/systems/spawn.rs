// systems/spawn.rs
use crate::components::{Actor, ActorType, Combat, Health, Movable, Position, Renderable};
use ggez::glam::Vec2;
use rand::Rng;
use specs::{Builder, System, World, Write};

pub struct SpawnSystem {
    pub spawn_timer: f32,
    pub spawn_interval: f32,
}

impl Default for SpawnSystem {
    fn default() -> Self {
        Self {
            spawn_timer: 0.0,
            spawn_interval: 5.0,
        }
    }
}

impl<'a> System<'a> for SpawnSystem {
    // 改用组合存储类型
    type SystemData = (specs::Entities<'a>, Write<'a, specs::LazyUpdate>);

    fn run(&mut self, (entities, lazy): Self::SystemData) {
        if self.spawn_timer >= self.spawn_interval {
            let mut rng = rand::thread_rng();

            let x = rng.gen_range(0.0..800.0);
            let y = rng.gen_range(0.0..600.0);
            println!("Spawning enemy at ({}, {})", x, y); // 添加调试输出
            // 使用 LazyUpdate 来创建实体
            lazy.create_entity(&entities)
                .with(Position { x, y, z: 5 })
                .with(Renderable {
                    path: "/images/box.png".to_string(),
                })
                .with(Actor {
                    speed: 150.0,
                    actor_type: ActorType::Enemy,
                })
                .with(Movable {
                    velocity: Vec2::ZERO,
                })
                .with(Health {
                    current: 50.0,
                    maximum: 50.0,
                })
                .with(Combat {
                    attack_damage: 10.0,
                    attack_cooldown: 1.0, // 敌人攻击较慢
                    attack_timer: 0.0,
                })
                .build();

            self.spawn_timer = 0.0;
            println!("Enemy spawned successfully"); // 添加调试输出
        }
    }
}
