use specs::Join;

pub struct DeleterSystem;

impl<'a> ::specs::System<'a> for DeleterSystem {
    type SystemData = (
        ::specs::WriteStorage<'a, ::component::DeletTimer>,
        ::specs::WriteStorage<'a, ::component::DeletBool>,
        ::specs::Fetch<'a, ::resource::UpdateTime>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (mut delet_timers, delet_bools, update_time, entities): Self::SystemData) {
        for (delet_timer, entity) in (&mut delet_timers, &*entities).join() {
            if delet_timer.0 <= 0.0 {
                entities.delete(entity).unwrap();
            }

            delet_timer.0 -= update_time.0;
        }

        for (delet_bool, entity) in (&delet_bools, &*entities).join() {
            if delet_bool.0 {
                entities.delete(entity).unwrap();
            }
        }
    }
}
