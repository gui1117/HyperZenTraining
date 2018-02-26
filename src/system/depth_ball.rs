use specs::Join;

pub struct DepthBallSystem;

impl<'a> ::specs::System<'a> for DepthBallSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Contactor>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::ReadStorage<'a, ::component::DepthBall>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::FetchMut<'a, ::resource::DepthCoef>,
        ::specs::FetchMut<'a, ::resource::Audio>,
    );

    fn run(&mut self, (contactors, players, depth_balls, mut lifes, mut depth_coef, mut audio): Self::SystemData) {
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
                audio.play(::audio::Sound::DepthBallDeath, contactor.contacts.first().unwrap().1.world1.coords.into());
            }
        }
    }
}
