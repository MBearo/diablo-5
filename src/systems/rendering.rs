// systems/rendering.rs
use crate::components::{Position, Renderable};
use ggez::glam::Vec2;
use ggez::{
    graphics::{self, DrawParam, Image},
    Context,
};
use specs::{Join, ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        let mut canvas =
            graphics::Canvas::from_frame(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image = Image::from_path(self.context, &renderable.path).expect("expected image");
            let x = position.x;
            let y = position.y;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            canvas.draw(&image, draw_params);
        }

        canvas.finish(self.context).expect("expected to present");
    }
}
