pub fn create_bouncer_w(
    pos: ::na::Vector3<f32>,
    eraser: bool,
    world: &mut ::specs::World,
) {
    create_bouncer(
        pos,
        eraser,
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write_resource(),
        &world.read_resource(),
        &world.read_resource()
    );
}

pub fn create_bouncer<'a>(
    pos: ::na::Vector3<f32>,
    eraser: bool,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    bouncers: &mut ::specs::WriteStorage<'a, ::component::Bouncer>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_erasers: &mut ::specs::WriteStorage<'a, ::component::DynamicEraser>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    contactors: &mut ::specs::WriteStorage<'a, ::component::Contactor>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let primitive_trans = ::graphics::resizer(config.bouncer_size, config.bouncer_size, config.bouncer_size);

    let shape = ::ncollide::shape::Ball3::new(config.bouncer_size);
    let pos = ::na::Isometry3::new(pos, ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::Sphere.instantiate();

    let entity = entities.create();
    bouncers.insert(entity, ::component::Bouncer);
    momentums.insert(entity, {
        let mut momentum =
            ::component::Momentum::new(mass, config.bouncer_velocity, config.bouncer_time_to_reach_vmax, None, config.bouncer_ang_damping, None);
        momentum.direction = ::na::Vector3::new_random().normalize();
        momentum
    });
    contactors.insert(entity, ::component::Contactor::new());
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            config.bouncer_color,
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

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
}
