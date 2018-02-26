use specs::Join;

pub struct ActivateSystem;

impl<'a> ::specs::System<'a> for ActivateSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Activated>,
        ::specs::ReadStorage<'a, ::component::Attracted>,
        ::specs::ReadStorage<'a, ::component::Avoider>,
        ::specs::ReadStorage<'a, ::component::Bouncer>,
        ::specs::ReadStorage<'a, ::component::Motionless>,
        ::specs::WriteStorage<'a, ::component::StaticDraw>,
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::FetchMut<'a, ::resource::Activated>,
    );

    fn run(&mut self, (activateds, attracted, avoider, bouncer, motionless, mut static_draws, audio, mut activated): Self::SystemData) {
        if !activated.0
            && attracted.join().next().is_none()
            && avoider.join().next().is_none()
            && bouncer.join().next().is_none()
            && motionless.join().next().is_none()
        {
            audio.play_unspatial(::audio::Sound::AllKilled);
            activated.0 = true;
            for (_, draw) in (&activateds, &mut static_draws).join() {
                draw.color = ::CONFIG.activated_color;
            }
        }
    }
}
