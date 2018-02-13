use specs::Join;

pub struct AttractedSystem {
    collided: Vec<(::specs::Entity, f32)>,
}

impl AttractedSystem {
    pub fn new() -> Self {
        AttractedSystem { collided: vec![] }
    }
}

impl<'a> ::specs::System<'a> for AttractedSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::WriteStorage<'a, ::component::Attracted>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::UpdateTime>,
    );

    fn run(&mut self, (players, bodies, mut attracteds, mut momentums, physic_world, update_time): Self::SystemData) {
        let player_pos = {
            let (_, player_body) = (&players, &bodies).join().next().unwrap();
            player_body.get(&physic_world).position().clone()
        };

        for (attracted, momentum, body) in (&mut attracteds, &mut momentums, &bodies).join() {
            let pos = body.get(&physic_world).position();
            attracted.last_update += update_time.0;

            while attracted.last_update >= ::CONFIG.attracted_update_time {
                attracted.last_update -= ::CONFIG.attracted_update_time;

                let ray = ::ncollide::query::Ray {
                    origin: ::na::Point3::from_coordinates(pos.translation.vector),
                    dir: player_pos.translation.vector - pos.translation.vector,
                };

                let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
                group.set_membership(&[::entity::ATTRACTED_VISION_GROUP]);
                group.set_whitelist(&[::entity::PLAYER_GROUP, ::entity::WALL_GROUP]);

                self.collided.clear();
                for (other_body, collision) in physic_world
                    .collision_world()
                    .interferences_with_ray(&ray, &group.as_collision_groups())
                {
                    if let ::nphysics::object::WorldObject::RigidBody(other_body) = other_body.data
                    {
                        let other_entity = ::component::PhysicBody::entity(physic_world.rigid_body(other_body));
                        self.collided.push((other_entity, collision.toi));
                    }
                }
                self.collided.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
                if self.collided.first().iter().any(|&&(e, _)| players.get(e).is_some()) {
                    momentum.direction = ray.dir;
                } else {
                    momentum.direction = ::na::zero();
                }
            }
        }
    }
}
