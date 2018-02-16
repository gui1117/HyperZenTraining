pub fn create_motionless_w(pos: ::na::Vector3<f32>, eraser: bool, world: &::specs::World) {
    create_motionless(
        pos,
        eraser,
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write_resource(),
        &world.read_resource(),
    );
}

pub fn create_motionless<'a>(
    pos: ::na::Vector3<f32>,
    eraser: bool,
    motionlesses: &mut ::specs::WriteStorage<'a, ::component::Motionless>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_erasers: &mut ::specs::WriteStorage<'a, ::component::DynamicEraser>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let primitive_trans = ::graphics::resizer(
        ::CONFIG.motionless_size,
        ::CONFIG.motionless_size,
        ::CONFIG.motionless_size,
    );

    let shape = ::ncollide::shape::Cuboid3::new(::na::Vector3::from_element(::CONFIG.motionless_size));
    let pos = ::na::Isometry3::new(pos, ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_static(shape, 0.0, 0.0);

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::Cube.instantiate();

    let entity = entities.create();
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            ::CONFIG.motionless_color,
            primitive_trans,
        ),
    );
    if eraser {
        dynamic_erasers.insert(entity, ::component::DynamicEraser);
        lifes.insert(entity, ::component::Life::EraserAlive);
    } else {
        lifes.insert(entity, ::component::Life::DrawAlive);
        dynamic_draws.insert(entity, ::component::DynamicDraw);
    }

    motionlesses.insert(entity, ::component::Motionless);

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
}
