use specs::Join;

pub struct TeleportSystem;

impl<'a> ::specs::System<'a> for TeleportSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Teleport>,
        ::specs::ReadStorage<'a, ::component::Proximitor>,
        ::specs::Fetch<'a, ::resource::Activated>,
        ::specs::FetchMut<'a, ::resource::LevelActions>,
    );

    fn run(&mut self, (teleports, proximitors, activated, mut level_actions): Self::SystemData) {
        if activated.0 {
            for (teleport, proximitor) in (&teleports, &proximitors).join() {
                if !proximitor.intersections.is_empty() {
                    level_actions.0.push(teleport.action.clone());
                }
            }
        }
    }
}
