use specs::Join;

pub struct LifeSystem;

impl<'a> ::specs::System<'a> for LifeSystem {
    type SystemData = (::specs::WriteStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::DynamicDraw>,
     ::specs::WriteStorage<'a, ::component::DynamicEraser>,
     ::specs::WriteStorage<'a, ::component::Life>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (mut bodies, mut dynamic_draws, mut dynamic_erasers, mut lives, mut physic_world, entities): Self::SystemData,
    ) {
        use component::Life;
        for (life, entity) in (&mut lives, &*entities).join() {
            match *life {
                Life::EraserDead => {
                    *life = Life::DrawAlive;
                    dynamic_draws.insert(entity, ::component::DynamicDraw);
                    dynamic_erasers.remove(entity).unwrap();
                }
                Life::DrawDead => {
                    if let Some(body) = bodies.get_mut(entity) {
                        body.remove(&mut physic_world);
                    }
                    entities.delete(entity).unwrap();
                }
                _ => (),
            }
        }
    }
}
