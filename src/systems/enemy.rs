use crate::components::{Actor, ActorType, EnemyAssets, EnemySpawnTimer, FlashEffect, HealthText};
use crate::util::{BOUNDARY_HEIGHT, BOUNDARY_WIDTH};
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

const INITIAL_ENEMY_NUM: i8 = 10;

// 在游戏启动时加载并存储敌人贴图
pub fn load_enemy_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_texture = asset_server.load("enemy.png");
    commands.insert_resource(EnemyAssets {
        texture: enemy_texture,
    });
}

pub fn spawn_enemies_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<&mut EnemySpawnTimer>,
    enemy_assets: Res<EnemyAssets>,
) {
    let mut timer = timer_query.single_mut();
    if timer.timer.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        spawn_enemy(&mut commands, &mut rng, &enemy_assets);
    }
}

// 生成单个敌人的函数
fn spawn_enemy(commands: &mut Commands, rng: &mut ThreadRng, enemy_assets: &EnemyAssets) {
    // 随机位置（在边界内）
    let half_width = (BOUNDARY_WIDTH / 2) as f32;
    let half_height = (BOUNDARY_HEIGHT / 2) as f32;
    let x = rng.gen_range(-half_width..half_width);
    let y = rng.gen_range(-half_height..half_height);

    commands
        .spawn((
            SpriteBundle {
                texture: enemy_assets.texture.clone(),
                sprite: Sprite {
                    // color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Actor::new_enemy(),
            // ShootingCooldown::default(),
        ))
        .with_children(|parent| {
            // 生成血量显示为子实体
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("HP: {}", Actor::new_enemy().health),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(1.0, 1.0, 1.0),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 30.0, 0.0), // 相对于父实体的位置
                    ..default()
                },
                HealthText,
            ));
        });
}

pub fn spawn_initial_enemies(mut commands: Commands, enemy_assets: Res<EnemyAssets>) {
    let mut rng = rand::thread_rng();
    for _ in 0..INITIAL_ENEMY_NUM {
        spawn_enemy(&mut commands, &mut rng, &enemy_assets);
    }
}

// 添加一个碰撞伤害常量
const ENEMY_COLLISION_DAMAGE: f32 = 20.0;
// 添加一个碰撞冷却时间（防止瞬间大量掉血）
const COLLISION_COOLDOWN: f32 = 0.5;

// 添加一个碰撞冷却组件
#[derive(Component, Clone)]
pub struct CollisionCooldown {
    pub timer: Timer,
}

impl Default for CollisionCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(COLLISION_COOLDOWN, TimerMode::Once),
        }
    }
}

// 单独的冷却更新系统
pub fn update_collision_cooldown(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut CollisionCooldown)>,
) {
    for (entity, mut cooldown) in query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<CollisionCooldown>();
        }
    }
}

pub fn enemy_player_collision_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query_set: ParamSet<(
        Query<(
            Entity,
            &Transform,
            &mut Actor,
            Option<&mut CollisionCooldown>,
        )>,
        Query<(&Transform, &Actor)>,
    )>,
) {
    let mut player_entity_opt = None;
    let mut should_take_damage = false;
    let mut player_position = Vec3::ZERO;

    // 第一次获取玩家查询
    {
        let player_query = query_set.p0();
        for (entity, transform, actor, cooldown) in player_query.iter() {
            if actor.actor_type == ActorType::Player {
                let mut can_take_damage = true;
                if let Some(cooldown) = cooldown {
                    // 正确处理计时器
                    let mut cooldown = cooldown.clone(); // 克隆计时器状态
                    cooldown.timer.tick(time.delta());
                    can_take_damage = cooldown.timer.finished();
                }

                if can_take_damage {
                    player_entity_opt = Some(entity);
                    player_position = transform.translation;
                }
                break;
            }
        }
    }

    // 检查与敌人的碰撞
    if let Some(_) = player_entity_opt {
        let enemy_query = query_set.p1();
        for (enemy_transform, enemy_actor) in enemy_query.iter() {
            if enemy_actor.actor_type == ActorType::Enemy {
                let distance = player_position.distance(enemy_transform.translation);

                if distance < 40.0 {
                    should_take_damage = true;
                    break;
                }
            }
        }
    }

    // 如果需要造成伤害
    if should_take_damage {
        if let Some(player_entity) = player_entity_opt {
            let mut player_query = query_set.p0();
            if let Ok((_, _, mut player_actor, cooldown)) = player_query.get_mut(player_entity) {
                // 再次检查冷却状态
                let can_take_damage = if let Some(cooldown) = cooldown {
                    cooldown.timer.finished()
                } else {
                    true
                };

                if can_take_damage {
                    player_actor.take_damage(ENEMY_COLLISION_DAMAGE);
                    commands
                        .entity(player_entity)
                        .insert(CollisionCooldown::default());
                    commands.entity(player_entity).insert(FlashEffect::new());

                    if !player_actor.is_alive() {
                        commands.entity(player_entity).despawn();
                    }
                }
            }
        }
    }
}

pub fn update_health_display(
    mut text_query: Query<(&mut Text, &Parent), With<HealthText>>,
    actor_query: Query<&Actor>,
) {
    for (mut text, parent) in text_query.iter_mut() {
        if let Ok(actor) = actor_query.get(parent.get()) {
            text.sections[0].value = format!("HP: {}", actor.health);
        }
    }
}


pub fn enemy_chase_system(
    player_query: Query<&Transform, With<Actor>>,
    mut enemy_query: Query<(&Transform, &mut Actor)>,
) {
    // 首先找到玩家
    let player_pos = player_query
        .iter()
        .find(|transform| true)  // 获取第一个玩家位置
        .map(|transform| transform.translation);

    if let Some(player_pos) = player_pos {
        // 更新每个敌人的移动方向
        for (enemy_transform, mut enemy_actor) in enemy_query.iter_mut() {
            if enemy_actor.actor_type == ActorType::Enemy {
                // 计算敌人到玩家的方向
                let enemy_pos = enemy_transform.translation;
                let direction = (player_pos - enemy_pos).normalize();
                
                // 更新敌人的移动方向
                enemy_actor.direction = Vec2::new(direction.x, direction.y);
            }
        }
    }
}
