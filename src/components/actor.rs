use bevy::prelude::*;

use super::InputController;

#[derive(PartialEq, Clone, Copy)]
pub enum BulletOwner {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub direction: Vec2,
}

// 定义角色类型的枚举
#[derive(PartialEq, Clone, Copy)]
pub enum ActorType {
    Player,
    Enemy,
    Bullet(BulletOwner),
}

impl InputController for Actor {}

#[derive(Component)]
pub struct Actor {
    // 移动速度
    pub speed: f32,
    // 移动方向 (使用 Vec2 表示 2D 向量)
    pub direction: Vec2,
    // 生命值
    pub health: f32,
    // 最大生命值
    pub max_health: f32,
    // 角色类型
    pub actor_type: ActorType,
}

impl Actor {
    pub fn new_bullet(owner: BulletOwner, direction: Vec2) -> Self {
        Self {
            speed: 600.0,
            direction,
            health: 1.0,
            max_health: 1.0,
            actor_type: ActorType::Bullet(owner),
        }
    }
    pub fn new_player() -> Self {
        Self {
            speed: 400.0,
            direction: Vec2::ZERO,
            health: 100.0,
            max_health: 100.0,
            actor_type: ActorType::Player,
        }
    }
    pub fn new_enemy() -> Self {
        Self {
            speed: 80.0,
            direction: Vec2::ZERO,
            health: 50.0,
            max_health: 50.0,
            actor_type: ActorType::Enemy,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    // 处理移动逻辑的方法
    pub fn handle_movement(&mut self, keys: &Res<ButtonInput<KeyCode>>, time: &Res<Time>) -> Vec3 {
        match self.actor_type {
            ActorType::Player => {
                // 使用 InputController 特征的默认实现
                self.direction = self.handle_input(keys);
            }
            ActorType::Enemy => {
                self.handle_enemy_movement();
            }
            ActorType::Bullet(_) => {
                // 子弹保持其方向不变，继续移动
                // direction 在生成时已经设置
            }
        }

        Vec3::new(self.direction.x, self.direction.y, 0.0) * self.speed * time.delta_seconds()
    }

    // 处理敌人移动逻辑
    fn handle_enemy_movement(&mut self) {
        // 这里可以实现敌人的 AI 移动逻辑
        // 比如追踪玩家、巡逻等
    }
}
