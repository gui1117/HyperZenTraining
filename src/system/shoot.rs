use specs::Join;
use alga::general::SubsetOf;

pub struct ShootSystem {
    collided: Vec<(::specs::Entity, f32)>,
}

impl ShootSystem {
    pub fn new() -> Self {
        ShootSystem { collided: vec![] }
    }
}

impl<'a> ::specs::System<'a> for ShootSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::ReadStorage<'a, ::component::WeaponAnimation>,
     ::specs::WriteStorage<'a, ::component::Shooter>,
     ::specs::WriteStorage<'a, ::component::Life>,
     ::specs::WriteStorage<'a, ::component::Deleter>,
     ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::WriteStorage<'a, ::component::DynamicDraw>,
     ::specs::WriteStorage<'a, ::component::DynamicHud>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (bodies, aims, animations, mut shooters, mut lifes, mut deleters, mut dynamic_assets, mut dynamic_draws, mut dynamic_huds, physic_world, config, entities): Self::SystemData,
    ) {
        for (aim, animation, body, shooter, entity) in (&aims, &animations, &bodies, &mut shooters, &*entities).join() {
            // Reload
            if shooter.bullets != shooter.max_bullets {
                shooter.timer += config.dt();
                if shooter.timer >= shooter.reload_time {
                    shooter.bullets += 1;
                    shooter.timer = 0.0;
                    dynamic_assets.get_mut(animation.bullets[shooter.bullets - 1]).unwrap().color = ::graphics::color::PALE_BLUE;
                }
            }

            // Shoot
            if shooter.shoot && shooter.bullets > 0 {
                shooter.bullets -= 1;
                shooter.shoot = false;
                dynamic_assets.get_mut(animation.bullets[shooter.bullets]).unwrap().color = ::graphics::color::DARK_BLUE;

                let body_pos = body.get(&physic_world).position().clone();

                let ray = ::ncollide::query::Ray {
                    origin: ::na::Point3::from_coordinates(body_pos.translation.vector),
                    dir: aim.dir,
                };

                // TODO: resolve hack with membership nphysic #82
                let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
                group.set_whitelist(&[::entity::MONSTER_GROUP, ::entity::WALL_GROUP]);

                self.collided.clear();
                for (other_body, collision) in
                    physic_world.collision_world().interferences_with_ray(
                        &ray,
                        &group.as_collision_groups(),
                    )
                {
                    if let ::nphysics::object::WorldObject::RigidBody(other_body) =
                        other_body.data
                    {
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
                let mut size = 1000.0; // infinite
                for collided in &self.collided {
                    if let Some(ref mut life) = lifes.get_mut(collided.0) {
                        life.kill();
                    } else {
                        size = collided.1;
                        break;
                    }
                }

                let aim_trans = {
                    let ah: ::na::Transform3<f32> =
                        ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -aim.x_dir)).to_superset();
                    let av: ::na::Transform3<f32> = ::na::Rotation3::new(
                        ::na::Vector3::new(0.0, -aim.dir[2].asin(), 0.0),
                    ).to_superset();
                    ah * av
                };
                let ray_draw_origin = (body_pos.translation * aim_trans * animation.weapon_trans * animation.shoot_pos).coords;
                let ray_draw_end = (ray.origin + size*ray.dir).coords;

                ::entity::create_light_ray(ray_draw_origin, ray_draw_end, animation.light_ray_radius, &mut deleters, &mut dynamic_draws, &mut dynamic_huds, &mut dynamic_assets, &entities);
            }
        }
    }
}
