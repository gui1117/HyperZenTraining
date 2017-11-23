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
    entities: &::specs::Entities,
) {
    let size = 0.05;

    let primitive_trans = ::graphics::resizer(size, size, size);

    let shape = ::ncollide::shape::Ball3::new(size);
    let pos = ::na::Isometry3::new(pos, ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();
    let velocity = 1.0;
    let time_to_reach_v_max = 0.05;
    let ang_damping = 0.8;

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::Sphere.instantiate();
    let color = ::graphics::color::BLUE;

    let entity = entities.create();
    bouncers.insert(entity, ::component::Bouncer);
    momentums.insert(entity, {
        let mut momentum =
            ::component::Momentum::new(mass, velocity, time_to_reach_v_max, ang_damping, None);
        momentum.direction = ::na::Vector3::new_random().normalize();
        momentum
    });
    contactors.insert(entity, ::component::Contactor::new());
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            color,
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
