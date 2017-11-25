use specs::Join;

pub struct FollowPlayerSystem;

impl<'a> ::specs::System<'a> for FollowPlayerSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::FollowPlayer>,
     ::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>);

    fn run(&mut self, (followers, players, bodies, mut momentums, physic_world): Self::SystemData) {
        let player_pos = (&players, &bodies)
            .join()
            .next()
            .unwrap()
            .1
            .get(&physic_world)
            .position()
            .translation
            .vector;

        for (_, body, momentum) in (&followers, &bodies, &mut momentums).join() {
            let pos = body.get(&physic_world).position().translation.vector;
            let vec = player_pos - pos;

            momentum.direction = vec.normalize();
        }
    }
}
