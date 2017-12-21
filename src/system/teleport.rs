use specs::Join;

pub struct TeleportSystem;

impl<'a> ::specs::System<'a> for TeleportSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Teleport>,
        ::specs::ReadStorage<'a, ::component::Proximitor>,
        ::specs::FetchMut<'a, ::resource::EndLevel>,
    );

    fn run(&mut self, (teleports, proximitors, mut end_level): Self::SystemData) {
        if (&teleports, &proximitors)
            .join()
            .any(|(_, p)| !p.intersections.is_empty())
        {
            end_level.0 = true;
        }
    }
}
