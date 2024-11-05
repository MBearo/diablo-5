use bevy::prelude::*;

use crate::components::{Actor, ActorType, Bullet, BulletAssets, BulletOwner, FlashEffect};

const SHOOTING_COOLDOWN: f32 = 1.0; // 射击冷却时间
const BULLET_SIZE: f32 = 10.0;
const BULLET_DAMAGE: f32 = 10.0;

#[derive(Component)]
pub struct ShootingCooldown {
    pub timer: Timer,
}

impl Default for ShootingCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SHOOTING_COOLDOWN, TimerMode::Once),
        }
    }
}

// 射击系统
pub fn shooting_system(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Transform, &mut ShootingCooldown, &Actor)>,
    targets_query: Query<(Entity, &Transform, &Actor)>,
    buttle_assets: Res<BulletAssets>,
) {
    for (entity, transform, mut cooldown, actor) in query.iter_mut() {
        cooldown.timer.tick(time.delta());

        match actor.actor_type {
            ActorType::Player => {
                if cooldown.timer.finished() {
                    // 找到任意一个敌人
                    if let Some((_, target_transform, _)) = targets_query
                        .iter()
                        .find(|(_, _, target)| target.actor_type == ActorType::Enemy)
                    {
                        spawn_bullet(
                            &mut commands,
                            transform.translation,
                            target_transform.translation,
                            BulletOwner::Player,
                            &buttle_assets,
                        );
                        cooldown.timer.reset();
                    }
                }
            }
            ActorType::Enemy => {
                if cooldown.timer.finished() {
                    // 找到玩家位置
                    if let Some((_, player_transform, _)) = targets_query
                        .iter()
                        .find(|(_, _, target)| target.actor_type == ActorType::Player)
                    {
                        spawn_bullet(
                            &mut commands,
                            transform.translation,
                            player_transform.translation,
                            BulletOwner::Enemy,
                            &buttle_assets,
                        );
                        cooldown.timer.reset();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn load_bullet_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bullet_texture = asset_server.load("bullet.png");
    commands.insert_resource(BulletAssets {
        texture: bullet_texture,
    });
}

fn spawn_bullet(
    commands: &mut Commands,
    start: Vec3,
    target: Vec3,
    owner: BulletOwner,
    buttle_assets: &BulletAssets,
) {
    let direction = (target - start).normalize().truncate();

    // 计算旋转角度
    let angle = direction.y.atan2(direction.x) + std::f32::consts::PI;

    commands.spawn((
        SpriteBundle {
            texture: buttle_assets.texture.clone(),
            sprite: Sprite {
                // color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(BULLET_SIZE * 1.5, BULLET_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(start)
                .with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
        Actor::new_bullet(owner, direction),
        Bullet {
            damage: BULLET_DAMAGE,
            direction,
        },
    ));
}

// 子弹碰撞检测系统
pub fn bullet_collision_system(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Actor, &Bullet)>,
    mut target_query: Query<(Entity, &Transform, &mut Actor), Without<Bullet>>,
) {
    for (bullet_entity, bullet_transform, bullet_actor, bullet) in bullet_query.iter() {
        if let ActorType::Bullet(owner) = bullet_actor.actor_type {
            for (target_entity, target_transform, mut target_actor) in target_query.iter_mut() {
                match (owner, target_actor.actor_type) {
                    (BulletOwner::Player, ActorType::Enemy)
                    | (BulletOwner::Enemy, ActorType::Player) => {
                        let distance = bullet_transform
                            .translation
                            .distance(target_transform.translation);

                        if distance < 30.0 {
                            // 碰撞检测范围
                            target_actor.take_damage(bullet.damage);
                            commands.entity(bullet_entity).despawn();

                            commands.entity(target_entity).insert(FlashEffect::new());

                            if !target_actor.is_alive() {
                                commands.entity(target_entity).despawn_recursive();
                            }
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
