use specs::Join;

pub struct PlayerDeathSystem;

impl<'a> ::specs::System<'a> for PlayerDeathSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Proximitor>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::FetchMut<'a, ::resource::LevelActions>,
    );

    fn run(&mut self, (proximitors, players, mut level_actions): Self::SystemData) {
        for (_, proximitor) in (&players, &proximitors).join() {
            if !proximitor.intersections.is_empty() {
                level_actions.0.push(::resource::LevelAction::Reset);
            }
        }
    }
}
