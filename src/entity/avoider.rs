pub fn create_avoider_w(pos: ::na::Vector3<f32>, eraser: bool, world: &::specs::World) {
    create_avoider(
        pos,
        eraser,
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

// IDEA: mabye make it turn on itself
pub fn create_avoider<'a>(
    pos: ::na::Vector3<f32>,
    eraser: bool,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    avoiders: &mut ::specs::WriteStorage<'a, ::component::Avoider>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_erasers: &mut ::specs::WriteStorage<'a, ::component::DynamicEraser>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let primitive_trans = ::graphics::resizer(
        ::CONFIG.avoider_size,
        ::CONFIG.avoider_size,
        ::CONFIG.avoider_size,
    );

    let shape = {
        let mut points = vec![
            ::na::Point3::new(-1.0, -1.0, -1.0),
            ::na::Point3::new(1.0, -1.0, -1.0),
            ::na::Point3::new(1.0, 1.0, -1.0),
            ::na::Point3::new(-1.0, 1.0, -1.0),
            ::na::Point3::new(0.0, 0.0, 1.0),
        ];
        for p in &mut points {
            *p = *p * ::CONFIG.avoider_size
        }
        ::ncollide::shape::ConvexHull::new(points)
    };

    let pos = ::na::Isometry3::new(pos, ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::MONSTER_GROUP, super::KILLER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();
    let pnt_to_com = ::na::Vector3::z() * ::CONFIG.avoider_size - body.center_of_mass().coords;

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::SquarePyramid.instantiate();

    let entity = entities.create();
    avoiders.insert(entity, ::component::Avoider::new());
    momentums.insert(
        entity,
        ::component::Momentum::new(
            mass,
            ::CONFIG.avoider_velocity,
            ::CONFIG.avoider_time_to_reach_vmax,
            None,
            ::CONFIG.avoider_ang_damping,
            ::na::Vector3::new(0.0, 0.0, 1.0),
            Some(pnt_to_com),
        ),
    );
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            ::CONFIG.avoider_color,
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
