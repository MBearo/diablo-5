use bevy::prelude::*;
mod actor;
mod input;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Boundary {
    pub height: i32,
    pub width: i32,
}

pub use self::{
    actor::{Actor, ActorType, Bullet, BulletOwner},
    input::InputController,
};

#[derive(Component)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct EnemyAssets {
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct BulletAssets {
    pub texture: Handle<Image>,
}


#[derive(Component)]
pub struct FlashEffect {
    pub timer: Timer,
    pub original_color: Color,
}

impl FlashEffect {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once), // 闪烁持续0.1秒
            original_color: Color::WHITE,
        }
    }
}

// 首先需要添加一个组件来标记血量文本
#[derive(Component)]
pub struct HealthText;
