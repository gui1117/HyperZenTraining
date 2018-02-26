use specs::Join;

pub struct PlayerDeathSystem;

impl<'a> ::specs::System<'a> for PlayerDeathSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Proximitor>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::FetchMut<'a, ::resource::LevelActions>,
    );

    fn run(&mut self, (proximitors, players, audio, mut level_actions): Self::SystemData) {
        for (_, proximitor) in (&players, &proximitors).join() {
            if !proximitor.intersections.is_empty() {
                audio.play_unspatial(::audio::Sound::Death);
                level_actions.0.push(::resource::LevelAction::Reset);
            }
        }
    }
}
