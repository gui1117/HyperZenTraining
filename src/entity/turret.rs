use std::f32::consts::FRAC_PI_2;

pub fn create_turret_w(pos: ::na::Vector3<f32>, world: &::specs::World) {
    create_turret(
        pos,
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write_resource(),
        &world.read_resource(),
    );
}

pub fn create_turret<'a>(
    pos: ::na::Vector3<f32>,
    turrets: &mut ::specs::WriteStorage<'a, ::component::Turret>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let primitive_trans =
        ::graphics::resizer(::CONFIG.turret_size, ::CONFIG.turret_size, ::CONFIG.turret_size);

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(
        ::CONFIG.turret_size,
        ::CONFIG.turret_size,
        ::CONFIG.turret_size,
    ));
    let trans = ::na::Isometry3::new(pos, ::na::Vector3::new(0.0, FRAC_PI_2, 0.0));

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::TURRET_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, ::CONFIG.turret_density, 0.0, 0.0);

    body.set_transformation(trans);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::PitCube.instantiate();

    let entity = entities.create();
    turrets.insert(
        entity,
        ::component::Turret::new(::CONFIG.turret_reload_time, pos)
    );

    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            ::CONFIG.turret_color,
            primitive_trans,
        ),
    );
    dynamic_draws.insert(entity, ::component::DynamicDraw);

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    bodies
        .get_mut(entity)
        .unwrap()
        .ball_in_socket(physic_world, ::na::Point3::from_coordinates(pos));
}
