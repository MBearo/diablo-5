mod components;
mod systems;
mod util;

use crate::systems::camera_movement;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use systems::{
    bullet_collision_system, enemy_chase_system, enemy_player_collision_system, flash_effect_system, fps_update_system, handle_movement, load_bullet_assets, load_enemy_assets, print_player_position, setup, shooting_system, spawn_enemies_system, spawn_initial_enemies, update_collision_cooldown, update_health_display
};

fn main() {
    App::new()
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup,
                load_enemy_assets,
                spawn_initial_enemies,
                load_bullet_assets,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                camera_movement,
                handle_movement,
                fps_update_system,
                spawn_enemies_system,
                shooting_system,
                bullet_collision_system,
                flash_effect_system,
                enemy_player_collision_system,
                update_collision_cooldown,
                update_health_display,
                enemy_chase_system
            ),
        )
        .insert_resource(Time::<Fixed>::from_seconds(1.0)) // 设置固定更新间隔为1秒
        .add_systems(FixedUpdate, print_player_position)
        .run();
}
