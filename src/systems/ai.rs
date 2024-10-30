pub struct AISystem {
    pub delta_time: f32,
}

impl<'a> System<'a> for AISystem {
    type SystemData = (
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Movable>,
    );

    fn run(&mut self, (actors, mut movables): Self::SystemData) {
        for (actor, movable) in (&actors, &mut movables).join() {
            if actor.actor_type == ActorType::Enemy {
                // 实现AI行为
                let mut rng = rand::thread_rng();
                movable.velocity = Vec2::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0)
                ).normalize();
            }
        }
    }
}