use std::f32::consts::FRAC_PI_2;

pub fn create_turret<'a>(
    pos: ::na::Vector3<f32>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    turrets: &mut ::specs::WriteStorage<'a, ::component::Turret>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    followers: &mut ::specs::WriteStorage<'a, ::component::FollowPlayer>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    // Create laser
    let laser_size = 0.01;
    let laser_shape = ::ncollide::shape::Ball3::new(laser_size);

    let mut laser_group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    laser_group.set_whitelist(&[super::TURRET_GROUP]);
    let mut laser_body = ::nphysics::object::RigidBody::new_dynamic(laser_shape, 1.0, 0.0, 0.0);

    laser_body.set_transformation(::na::Isometry3::new(
        pos + ::na::Vector3::new(1.0, 0.0, 0.0),
        ::na::zero(),
    ));
    laser_body.set_collision_groups(laser_group);

    let laser_mass = 1.0 / laser_body.inv_mass();

    let laser_physic_entity = entities.create();
    followers.insert(laser_physic_entity, ::component::FollowPlayer::new(config.laser_amortization));
    momentums.insert(
        laser_physic_entity,
        ::component::Momentum::new(
            laser_mass,
            config.laser_velocity,
            config.laser_time_to_reach_vmax,
            None,
            config.laser_ang_damping,
            None,
        ),
    );
    ::component::PhysicBody::add(laser_physic_entity, laser_body, bodies, physic_world);

    let (laser_primitive, laser_groups) = ::graphics::Primitive::Cylinder.instantiate();
    let laser_draw_entity = entities.create();
    dynamic_graphics_assets.insert(
        laser_draw_entity,
        ::component::DynamicGraphicsAssets::new(
            laser_primitive,
            laser_groups,
            config.laser_color,
            ::na::one(),
        ),
    );
    dynamic_draws.insert(laser_draw_entity, ::component::DynamicDraw);

    // Create turret
    let primitive_trans = ::graphics::resizer(config.turret_size, config.turret_size, config.turret_size);

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(config.turret_size, config.turret_size, config.turret_size));
    let trans = ::na::Isometry3::new(pos, ::na::Vector3::new(0.0, FRAC_PI_2, 0.0));

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 10.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();

    body.set_transformation(trans);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::PitCube.instantiate();

    let entity = entities.create();
    turrets.insert(
        entity,
        ::component::Turret {
            laser_draw: laser_draw_entity,
            laser_physic: laser_physic_entity,
        },
    );
    momentums.insert(
        entity,
        ::component::Momentum::new(
            mass,
            config.turret_velocity,
            config.turret_time_to_reach_vmax,
            None,
            config.turret_ang_damping,
            None,
        ),
    );

    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            config.turret_color,
            primitive_trans,
        ),
    );
    lifes.insert(entity, ::component::Life::DrawAlive);
    dynamic_draws.insert(entity, ::component::DynamicDraw);

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    bodies.get_mut(entity).unwrap().ball_in_socket(
        physic_world,
        ::na::Point3::from_coordinates(pos),
    );
}
