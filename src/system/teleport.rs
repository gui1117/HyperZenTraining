use specs::Join;

pub struct TeleportSystem;

impl<'a> ::specs::System<'a> for TeleportSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Teleport>,
        ::specs::ReadStorage<'a, ::component::Proximitor>,
        ::specs::FetchMut<'a, ::resource::LevelActions>,
    );

    fn run(&mut self, (teleports, proximitors, mut level_actions): Self::SystemData) {
        for (teleport, proximitor) in (&teleports, &proximitors).join() {
            if !proximitor.intersections.is_empty() {
                level_actions.0.push(teleport.action.clone());
            }
        }
    }
}
