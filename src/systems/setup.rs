use std::time::Duration;

use crate::components::Actor;
use crate::components::Boundary;
use crate::components::EnemySpawnTimer;
use crate::components::FpsText;
use crate::components::HealthText;
use crate::util::{BOUNDARY_HEIGHT, BOUNDARY_WIDTH};
use bevy::{color::palettes::css::GOLD, prelude::*};

use super::shooting::ShootingCooldown;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 添加2D摄像机
    commands.spawn(Camera2dBundle::default());

    // 生成背景网格
    let grid_size = 100.0; // 每个网格的大小
    let grid_count = 20; // 网格数量（横向和纵向）

    // 生成背景网格
    spawn_grid(&mut commands, grid_size, grid_count);

    // 生成边界线（红色正方形）
    spawn_boundary(&mut commands, BOUNDARY_WIDTH, BOUNDARY_HEIGHT);

    let player_texture = asset_server.load("player.png");
    // 生成玩家方块
    commands
        .spawn((
            SpriteBundle {
                texture: player_texture,
                sprite: Sprite {
                    // color: Color::srgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Actor::new_player(),
            Boundary {
                height: BOUNDARY_HEIGHT,
                width: BOUNDARY_WIDTH,
            },
            ShootingCooldown::default(),
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
    commands.spawn(EnemySpawnTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
    });

    // FPS
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: GOLD.into(),
                ..default()
            }),
        ]),
        FpsText,
    ));
}

// 生成网格的辅助函数
fn spawn_grid(commands: &mut Commands, grid_size: f32, grid_count: i32) {
    // 生成横线
    for i in -grid_count..=grid_count {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.5, 0.5, 0.5, 0.3), // 灰色半透明
                custom_size: Some(Vec2::new(grid_size * grid_count as f32 * 2.0, 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, i as f32 * grid_size, -1.0),
            ..default()
        });
    }

    // 生成竖线
    for i in -grid_count..=grid_count {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.5, 0.5, 0.5, 0.3),
                custom_size: Some(Vec2::new(2.0, grid_size * grid_count as f32 * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(i as f32 * grid_size, 0.0, -1.0),
            ..default()
        });
    }
}

// 生成边界的辅助函数
fn spawn_boundary(commands: &mut Commands, width: i32, height: i32) {
    // 转换为 f32
    let width_half_size = (width / 2) as f32;
    let height_half_size = (height / 2) as f32;
    let width_f32 = width as f32;
    let height_f32 = height as f32;
    let line_thickness = 4.0;

    // 上边界
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(width_f32, line_thickness)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, height_half_size, 0.0),
        ..default()
    });

    // 下边界
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(width_f32, line_thickness)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -height_half_size, 0.0),
        ..default()
    });

    // 左边界
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(line_thickness, height_f32)),
            ..default()
        },
        transform: Transform::from_xyz(-width_half_size, 0.0, 0.0),
        ..default()
    });

    // 右边界
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(line_thickness, height_f32)),
            ..default()
        },
        transform: Transform::from_xyz(width_half_size, 0.0, 0.0),
        ..default()
    });
}
