use bevy::prelude::*;

use crate::components::FlashEffect;

pub fn flash_effect_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FlashEffect, &mut Sprite)>,
) {
    for (entity, mut flash, mut sprite) in query.iter_mut() {
        flash.timer.tick(time.delta());
        
        if !flash.timer.finished() {
            // 闪烁期间将颜色设置为红色
            sprite.color = Color::srgb(1.0, 0.0, 0.0);
        } else {
            // 闪烁结束后恢复原始颜色
            sprite.color = flash.original_color;
            // 移除闪烁组件
            commands.entity(entity).remove::<FlashEffect>();
        }
    }
}