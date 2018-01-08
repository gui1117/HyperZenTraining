use specs::Join;

pub struct ShootSystem {
    collided: Vec<(::specs::Entity, f32)>,
}

impl ShootSystem {
    pub fn new() -> Self {
        ShootSystem { collided: vec![] }
    }
}

impl<'a> ::specs::System<'a> for ShootSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::ReadStorage<'a, ::component::WeaponAnimation>,
        ::specs::WriteStorage<'a, ::component::Shooter>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::WriteStorage<'a, ::component::DeletTimer>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::UpdateTime>,
        ::specs::Entities<'a>,
    );

    fn run(
        &mut self,
        (
            bodies,
            aims,
            animations,
            mut shooters,
            mut lifes,
            mut delet_timers,
            mut dynamic_assets,
            mut dynamic_draws,
            physic_world,
            update_time,
            entities,
        ): Self::SystemData,
    ) {
        for (aim, animation, body, shooter, entity) in
            (&aims, &animations, &bodies, &mut shooters, &*entities).join()
        {
            // Reload
            if shooter.bullets != shooter.max_bullets {
                shooter.timer += update_time.0;
                if shooter.timer >= shooter.reload_time {
                    shooter.bullets += 1;
                    shooter.timer = 0.0;
                    dynamic_assets
                        .get_mut(animation.bullets[shooter.bullets - 1])
                        .unwrap()
                        .color = ::graphics::Color::PaleBlue;
                }
            }

            // Shoot
            if shooter.shoot && shooter.bullets > 0 {
                shooter.bullets -= 1;
                shooter.shoot = false;
                dynamic_assets
                    .get_mut(animation.bullets[shooter.bullets])
                    .unwrap()
                    .color = ::graphics::Color::DarkBlue;

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
                let mut size = 1000.0; // infinite
                for collided in &self.collided {
                    if let Some(ref mut life) = lifes.get_mut(collided.0) {
                        life.kill();
                    } else {
                        size = collided.1;
                        break;
                    }
                }

                let ray_draw_origin = (body_pos.translation * aim.rotation * animation.weapon_trans
                    * animation.shoot_pos)
                    .coords;
                let ray_draw_end = (ray.origin + size * ray.dir).coords;

                ::entity::create_light_ray(
                    ray_draw_origin,
                    ray_draw_end,
                    animation.light_ray_radius,
                    &mut delet_timers,
                    &mut dynamic_draws,
                    &mut dynamic_assets,
                    &entities,
                );
            }
        }
    }
}
