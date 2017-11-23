use specs::Join;

pub struct DeleterSystem;

impl<'a> ::specs::System<'a> for DeleterSystem {
    type SystemData = (::specs::WriteStorage<'a, ::component::Deleter>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::Entities<'a>);

    fn run(&mut self, (mut deleters, config, entities): Self::SystemData) {
        for (deleter, entity) in (&mut deleters, &*entities).join() {
            if deleter.timer <= 0.0 {
                entities.delete(entity).unwrap();
            }

            deleter.timer -= config.dt();
        }
    }
}
