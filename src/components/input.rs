use bevy::prelude::*;

// 定义输入控制特征
pub trait InputController {
    fn handle_input(&mut self, keys: &Res<ButtonInput<KeyCode>>) -> Vec2 {
        let mut direction = Vec2::ZERO;

        if keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        // 标准化方向向量
        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        direction
    }
}

// // 实现标准的键盘方向键控制
// pub struct KeyboardArrowController;

// impl InputController for KeyboardArrowController {
//     fn handle_input(&mut self, keys: &Res<ButtonInput<KeyCode>>) -> Vec2 {
//         let mut direction = Vec2::ZERO;

//         if keys.pressed(KeyCode::ArrowLeft) {
//             direction.x -= 1.0;
//         }
//         if keys.pressed(KeyCode::ArrowRight) {
//             direction.x += 1.0;
//         }
//         if keys.pressed(KeyCode::ArrowUp) {
//             direction.y += 1.0;
//         }
//         if keys.pressed(KeyCode::ArrowDown) {
//             direction.y -= 1.0;
//         }

//         // 标准化方向向量
//         if direction != Vec2::ZERO {
//             direction = direction.normalize();
//         }

//         direction
//     }
// }

// // 可以实现其他控制方式,比如WASD控制
// pub struct WASDController;

// impl InputController for WASDController {
//     fn handle_input(&mut self, keys: &Res<ButtonInput<KeyCode>>) -> Vec2 {
//         let mut direction = Vec2::ZERO;

//         if keys.pressed(KeyCode::KeyA) {
//             direction.x -= 1.0;
//         }
//         if keys.pressed(KeyCode::KeyD) {
//             direction.x += 1.0;
//         }
//         if keys.pressed(KeyCode::KeyW) {
//             direction.y += 1.0;
//         }
//         if keys.pressed(KeyCode::KeyS) {
//             direction.y -= 1.0;
//         }

//         if direction != Vec2::ZERO {
//             direction = direction.normalize();
//         }

//         direction
//     }
// }
