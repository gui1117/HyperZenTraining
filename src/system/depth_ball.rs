use specs::Join;

pub struct DepthBallSystem;

impl<'a> ::specs::System<'a> for DepthBallSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Contactor>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::ReadStorage<'a, ::component::DepthBall>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::FetchMut<'a, ::resource::DepthCoef>,
    );

    fn run(&mut self, (contactors, players, depth_balls, mut lifes, audio, mut depth_coef): Self::SystemData) {
        for (_, life, contactor) in (&depth_balls, &mut lifes, &contactors).join() {
            if contactor.contacts.is_empty() {
                continue;
            }

            life.kill();
            let mut attack = false;
            if contactor.contacts.iter().any(|&(e, _)| players.get(e).is_some()) {
                depth_coef.0 /= ::CONFIG.depth_coef_divider;
                attack = true;
                audio.play_unspatial(::audio::Sound::DepthBallAttack);
            }

            if !attack {
                audio.play(::audio::Sound::DepthBallBirthDeath, contactor.contacts.first().unwrap().1.world1.coords.into());
            }
        }
    }
}
