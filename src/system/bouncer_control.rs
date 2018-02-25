use specs::Join;

pub struct BouncerControlSystem;

impl<'a> ::specs::System<'a> for BouncerControlSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Contactor>,
        ::specs::ReadStorage<'a, ::component::Bouncer>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::FetchMut<'a, ::resource::Audio>,
    );

    fn run(&mut self, (contactors, bouncers, mut momentums, mut audio): Self::SystemData) {
        for (_, momentum, contactor) in (&bouncers, &mut momentums, &contactors).join() {
            if contactor.contacts.is_empty() {
                continue;
            }

            let mut normal = ::na::Vector3::new(0.0, 0.0, 0.0);
            let mut point = ::na::Vector3::new(0.0, 0.0, 0.0);
            for &(_, ref contact) in &contactor.contacts {
                normal -= contact.depth * contact.normal;
                point += contact.world1.coords;
            }
            point /= contactor.contacts.len() as f32;
            audio.play(::audio::Sound::Bounce, point.into());

            normal.normalize_mut();
            let proj_on_normal = momentum.direction.dot(&normal) * normal;
            if proj_on_normal.dot(&normal) > 0.0 {
                momentum.direction -= 2.0 * proj_on_normal;
            }
        }
    }
}
