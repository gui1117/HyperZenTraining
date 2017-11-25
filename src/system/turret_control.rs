use specs::Join;

pub struct TurretControlSystem {
    collided: Vec<(::specs::Entity, f32)>,
}

impl TurretControlSystem {
    pub fn new() -> Self {
        TurretControlSystem { collided: vec![] }
    }
}

impl<'a> ::specs::System<'a> for TurretControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Turret>,
     ::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::FetchMut<'a, ::resource::DepthCoef>,
     ::specs::Entities<'a>);

    fn run(&mut self, (turrets, players, bodies, mut momentums, mut assets, physic_world, mut depth_coef, entities): Self::SystemData) {
        let depth_coef_velocity = 1.05;
        let depth_coef_min = 0.001;
        let ray_radius = 0.01;

        depth_coef.0 *= depth_coef_velocity;

        // Update turrets
        for (turret, body, momentum, entity) in
            (&turrets, &bodies, &mut momentums, &*entities).join()
        {
            let pos = body.get(&physic_world).position();
            let laser_pos = bodies
                .get(turret.laser_physic)
                .unwrap()
                .get(&physic_world)
                .position();

            momentum.ang_force = Some(pos.rotation.rotation_to(&::na::UnitQuaternion::rotation_between(&::na::Vector3::new(0.0, 0.0, 1.0), &(laser_pos.translation.vector-pos.translation.vector)).unwrap_or(::na::UnitQuaternion::new(::na::zero()))).scaled_axis());

            let shoot_dir = (pos.rotation * ::na::Point3::new(0.0, 0.0, 1.0)).coords;

            // TODO: factorise raycast
            let ray = ::ncollide::query::Ray {
                origin: ::na::Point3::from_coordinates(pos.translation.vector),
                dir: shoot_dir,
            };

            let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
            group.set_whitelist(&[::entity::ALIVE_GROUP, ::entity::WALL_GROUP]);

            self.collided.clear();
            for (other_body, collision) in
                physic_world.collision_world().interferences_with_ray(
                    &ray,
                    &group.as_collision_groups(),
                )
            {
                if let ::nphysics::object::WorldObject::RigidBody(other_body) = other_body.data {
                    let other_entity =
                        ::component::PhysicBody::entity(physic_world.rigid_body(other_body));
                    if entity != other_entity {
                        self.collided.push((other_entity, collision.toi));
                    }
                }
            }
            self.collided.sort_by(
                |a, b| (a.1).partial_cmp(&b.1).unwrap(),
            );
            let ray_length = if let Some(collided) = self.collided.first() {
                if players.get(collided.0).is_some() {
                    depth_coef.0 /= depth_coef_velocity.powi(2);
                }
                collided.1
            } else {
                1000.0
            };

            let world_trans = ::na::Isometry3::from_parts(
                ::na::Translation::from_vector(pos.translation.vector + ray_length/2.0*shoot_dir),
                pos.rotation,
            ) *
                ::graphics::resizer(ray_radius, ray_radius, ray_length / 2.0);
            assets.get_mut(turret.laser_draw).unwrap().world_trans =
                ::graphics::shader::draw1_vs::ty::World { world: world_trans.unwrap().into() };
        }

        depth_coef.0 = depth_coef.0.min(1.0).max(depth_coef_min);
    }
}
