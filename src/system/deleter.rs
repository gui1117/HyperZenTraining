use specs::Join;

pub struct DeleterSystem;

impl<'a> ::specs::System<'a> for DeleterSystem {
    type SystemData = (
        ::specs::WriteStorage<'a, ::component::DeletTimer>,
        ::specs::WriteStorage<'a, ::component::DeletBool>,
        ::specs::Fetch<'a, ::resource::Config>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (mut delet_timers, delet_bools, config, entities): Self::SystemData) {
        for (delet_timer, entity) in (&mut delet_timers, &*entities).join() {
            if delet_timer.0 <= 0.0 {
                entities.delete(entity).unwrap();
            }

            delet_timer.0 -= config.dt();
        }

        for (delet_bool, entity) in (&delet_bools, &*entities).join() {
            if delet_bool.0 {
                entities.delete(entity).unwrap();
            }
        }
    }
}
