use specs::Join;

pub struct ReducerSystem;

impl<'a> ::specs::System<'a> for ReducerSystem {
    type SystemData = (
        ::specs::WriteStorage<'a, ::component::Reducer>,
        ::specs::Fetch<'a, ::resource::UpdateTime>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (mut reducers, update_time, entities): Self::SystemData) {
        for (reducer, entity) in (&mut reducers, &*entities).join() {
            if reducer.timer > reducer.duration {
                entities.delete(entity).unwrap();
            }

            reducer.timer += update_time.0;
        }
    }
}
