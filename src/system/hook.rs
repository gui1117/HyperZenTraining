use specs::Join;

pub struct HookSystem {
    collided: Vec<(::specs::Entity, f32)>,
}

impl HookSystem {
    pub fn new() -> Self {
        HookSystem { collided: vec![] }
    }
}

impl<'a> ::specs::System<'a> for HookSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::WriteStorage<'a, ::component::Hook>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (bodies, aims, mut hooks, physic_world, entities): Self::SystemData) {
        for (aim, body, hook, entity) in (&aims, &bodies, &mut hooks, &*entities).join() {
            // Delete anchor if entity doesn't exist anymore
            if let Some(false) = hook.anchor
                .as_ref()
                .map(|anchor| entities.is_alive(anchor.entity))
            {
                hook.anchor = None;
                hook.launch = false;
            }

            if !hook.launch && hook.anchor.is_some() {
                hook.anchor = None;
            }

            if hook.launch && hook.anchor.is_none() {
                let body_pos = body.get(&physic_world).position().clone();

                let ray = ::ncollide::query::Ray {
                    origin: ::na::Point3::from_coordinates(body_pos.translation.vector),
                    dir: aim.rotation * ::na::Vector3::x(),
                };

                // TODO: resolve hack with membership nphysic #82
                let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
                group.set_whitelist(&[::entity::MONSTER_GROUP, ::entity::WALL_GROUP]);

                self.collided.clear();
                for (other_body, collision) in physic_world
                    .collision_world()
                    .interferences_with_ray(&ray, &group.as_collision_groups())
                {
                    if let ::nphysics::object::WorldObject::RigidBody(other_body) = other_body.data
                    {
                        let other_entity =
                            ::component::PhysicBody::entity(physic_world.rigid_body(other_body));
                        if entity != other_entity {
                            self.collided.push((other_entity, collision.toi));
                        }
                    }
                }
                self.collided
                    .sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
                for collided in &self.collided {
                    let other_pos = bodies
                        .get(collided.0)
                        .unwrap()
                        .get(&physic_world)
                        .position();
                    let collision_pos = ray.origin + ray.dir * collided.1;
                    let local_pos = other_pos.inverse() * collision_pos;
                    hook.anchor = Some(::component::Anchor {
                        entity: collided.0,
                        local_pos,
                        pos: ::na::zero(),
                    });
                    break;
                }
            }

            // compute position and draw
            if let Some(ref mut anchor) = hook.anchor {
                anchor.pos = (bodies
                    .get(anchor.entity)
                    .unwrap()
                    .get(&physic_world)
                    .position() * anchor.local_pos)
                    .coords;
            }
        }
    }
}
