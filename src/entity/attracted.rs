use rand::distributions::{IndependentSample, Range};

pub fn create_attracted_w(pos: ::na::Vector3<f32>, eraser: bool, world: &::specs::World) {
    create_attracted(
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
    );
}

pub fn create_attracted<'a>(
    pos: ::na::Vector3<f32>,
    eraser: bool,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    attracteds: &mut ::specs::WriteStorage<'a, ::component::Attracted>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_erasers: &mut ::specs::WriteStorage<'a, ::component::DynamicEraser>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    contactors: &mut ::specs::WriteStorage<'a, ::component::Contactor>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let primitive_trans = ::graphics::resizer(
        ::CONFIG.attracted_size,
        ::CONFIG.attracted_size,
        ::CONFIG.attracted_size,
    );

    let shape = ::ncollide::shape::Ball3::new(::CONFIG.attracted_size);
    let mut pos = ::na::Isometry3::new(pos, ::na::zero());

    let mut rng = ::rand::thread_rng();
    pos.translation.vector += ::na::Vector3::new(
        Range::new(-0.5, 0.5).ind_sample(&mut rng),
        Range::new(-0.5, 0.5).ind_sample(&mut rng),
        Range::new(-0.4, 0.4).ind_sample(&mut rng),
    );

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP, super::KILLER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::Sphere.instantiate();

    let entity = entities.create();
    attracteds.insert(entity, ::component::Attracted::new());
    momentums.insert(entity, {
        let mut momentum = ::component::Momentum::new(
            mass,
            ::CONFIG.attracted_velocity,
            ::CONFIG.attracted_time_to_reach_vmax,
            None,
            ::CONFIG.attracted_ang_damping,
            ::na::zero(),
            None,
        );
        momentum.direction = ::na::Vector3::new_random().normalize();
        momentum
    });
    contactors.insert(entity, ::component::Contactor::new());
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            ::CONFIG.attracted_color,
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
