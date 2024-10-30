use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{event, Context, GameResult};
use specs::{Builder, RunNow, World, WorldExt};

use crate::components::*;
use crate::systems::{
    CollisionSystem, CombatSystem, InputSystem, MovementSystem, RenderingSystem, SpawnSystem,
};

pub struct Game {
    world: World,
    input_system: InputSystem,
    movement_system: MovementSystem,
    spawn_system: SpawnSystem,
    combat_system: CombatSystem,
    collision_system: CollisionSystem,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        register_components(&mut world);
        // initialize_level(&mut world);

        // 创建玩家
        world
            .create_entity()
            .with(Position {
                x: 400.0,
                y: 300.0,
                z: 10,
            })
            .with(Renderable {
                path: "/images/player.png".to_string(),
            })
            .with(Actor {
                speed: 200.0,
                actor_type: ActorType::Player,
            })
            .with(Movable {
                velocity: Vec2::ZERO,
            })
            .with(Combat {
                attack_damage: 20.0,
                attack_cooldown: 0.5, // 玩家攻击较快
                attack_timer: 0.0,
            })
            .with(Health {
                current: 100.0,
                maximum: 100.0,
            })
            .build();

        Self {
            world,
            input_system: InputSystem::default(),
            movement_system: MovementSystem { delta_time: 0.0 },
            spawn_system: SpawnSystem::default(),
            combat_system: CombatSystem { delta_time: 0.0 },
            collision_system: CollisionSystem,
        }
    }
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // 帧间隔
        let delta = context.time.delta().as_secs_f32();
        // 更新所有系统
        self.movement_system.delta_time = delta;
        self.movement_system.run_now(&self.world);

        self.input_system.delta_time = context.time.delta().as_secs_f32();
        self.input_system.run_now(&self.world);

        self.spawn_system.spawn_timer += delta;
        self.spawn_system.run_now(&self.world);

        self.combat_system.delta_time = delta;
        self.combat_system.run_now(&self.world);

        self.collision_system.run_now(&self.world);

        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rs = RenderingSystem { context };
        rs.run_now(&self.world);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => self.input_system.up_pressed = true,
            Some(KeyCode::Down) => self.input_system.down_pressed = true,
            Some(KeyCode::Left) => self.input_system.left_pressed = true,
            Some(KeyCode::Right) => self.input_system.right_pressed = true,
            _ => (),
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => self.input_system.up_pressed = false,
            Some(KeyCode::Down) => self.input_system.down_pressed = false,
            Some(KeyCode::Left) => self.input_system.left_pressed = false,
            Some(KeyCode::Right) => self.input_system.right_pressed = false,
            _ => (),
        }
        Ok(())
    }
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Actor>();
    world.register::<Movable>();
    world.register::<Combat>();
    world.register::<Health>();
    world.register::<Bullet>();
}

// pub fn create_player(world: &mut World, position: Position) {
//     world
//         .create_entity()
//         .with(Position { z: 10, ..position })
//         .with(Renderable {
//             path: "/images/player.png".to_string(),
//         })
//         .with(Player {})
//         .build();
// }

// pub fn initialize_level(world: &mut World) {
//     create_player(
//         world,
//         Position {
//             x: 0.0,
//             y: 0.0,
//             z: 0,
//         },
//     );
// }
