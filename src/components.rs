use ggez::glam::Vec2;
use specs::{Component, VecStorage};

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {}

// 实体类型
#[derive(Component)]
#[storage(VecStorage)]
pub struct Actor {
    pub speed: f32,
    pub actor_type: ActorType,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ActorType {
    Player,
    Enemy,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Movable {
    pub velocity: Vec2,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Bullet {
    pub damage: f32,
    pub owner_type: ActorType, // 用来区分是谁发射的子弹
    // pub speed: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Combat {
    pub attack_damage: f32,
    pub attack_cooldown: f32,
    pub attack_timer: f32,
}
