use crate::components::{Actor, Bullet, Movable, Position};
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct MovementSystem {
    pub delta_time: f32,
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Movable>,
        ReadStorage<'a, Bullet>, // 添加 Bullet 组件的读取
    );

    fn run(&mut self, (mut positions, actors, mut movables, bullets): Self::SystemData) {
        // 处理带 Actor 的实体(玩家和敌人)
        for (pos, actor, movable) in (&mut positions, &actors, &mut movables).join() {
            pos.x += movable.velocity.x * actor.speed * self.delta_time;
            pos.y += movable.velocity.y * actor.speed * self.delta_time;

            // 保持在屏幕范围内
            pos.x = pos.x.clamp(0.0, 800.0);
            pos.y = pos.y.clamp(0.0, 600.0);
        }

        // 处理子弹的移动
        for (pos, movable, _bullet) in (&mut positions, &mut movables, &bullets).join() {
            // 子弹直接使用 velocity,不需要乘以 speed
            pos.x += movable.velocity.x * self.delta_time;
            pos.y += movable.velocity.y * self.delta_time;
        }
    }
}
