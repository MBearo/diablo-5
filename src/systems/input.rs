// systems/input.rs
use crate::components::{Actor, ActorType, Movable};
use ggez::glam::Vec2;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct InputSystem {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub delta_time: f32,
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            delta_time: 0.0,
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadStorage<'a, Actor>, WriteStorage<'a, Movable>);

    fn run(&mut self, (actors, mut movables): Self::SystemData) {
        for (actor, movable) in (&actors, &mut movables).join() {
            if actor.actor_type == ActorType::Player {
                let mut velocity = Vec2::ZERO;

                if self.left_pressed {
                    velocity.x -= 1.0;
                }
                if self.right_pressed {
                    velocity.x += 1.0;
                }
                if self.up_pressed {
                    velocity.y -= 1.0;
                }
                if self.down_pressed {
                    velocity.y += 1.0;
                }

                if velocity != Vec2::ZERO {
                    velocity = velocity.normalize();
                }

                movable.velocity = velocity;
            }
        }
    }
}
