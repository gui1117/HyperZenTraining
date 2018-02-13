#[allow(unused)]
pub fn create_depth_ball_w(pos: ::na::Vector3<f32>, dir: ::na::Vector3<f32>, world: &::specs::World) {
    create_depth_ball(
        pos,
        dir,
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write_resource(),
        &world.read_resource(),
    )
}

pub fn create_depth_ball<'a>(
    pos: ::na::Vector3<f32>,
    dir: ::na::Vector3<f32>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    depth_balls: &mut ::specs::WriteStorage<'a, ::component::DepthBall>,
    contactors: &mut ::specs::WriteStorage<'a, ::component::Contactor>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let primitive_trans = ::graphics::resizer(
        ::CONFIG.depth_ball_size,
        ::CONFIG.depth_ball_size,
        ::CONFIG.depth_ball_size,
    );

    let shape = ::ncollide::shape::Ball3::new(::CONFIG.depth_ball_size);
    let pos = ::na::Isometry3::new(pos, ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP]);
    group.set_blacklist(&[super::TURRET_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::Sphere.instantiate();

    let entity = entities.create();
    depth_balls.insert(entity, ::component::DepthBall);
    momentums.insert(entity, {
        let mut momentum = ::component::Momentum::new(
            mass,
            ::CONFIG.depth_ball_velocity,
            ::CONFIG.depth_ball_time_to_reach_vmax,
            None,
            ::CONFIG.depth_ball_ang_damping,
            ::na::zero(),
            None,
        );
        momentum.direction = dir.normalize();
        momentum
    });
    contactors.insert(entity, ::component::Contactor::new());
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            ::CONFIG.depth_ball_color,
            primitive_trans,
        ),
    );
    lifes.insert(entity, ::component::Life::DrawAlive);
    dynamic_draws.insert(entity, ::component::DynamicDraw);

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
}
