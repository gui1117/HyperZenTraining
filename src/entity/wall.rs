use alga::general::SubsetOf;

pub fn create_wall_side<'a>(
    pos: ::na::Isometry3<f32>,
    x_radius: f32,
    y_radius: f32,
    color: Option<::graphics::Color>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[super::WALL_GROUP]);

    let world_trans = {
        let pos_trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 1.0)
            .to_superset();
        let trans = pos_trans * ::graphics::resizer(x_radius, y_radius, 1.0);
        ::graphics::shader::draw1_vs::ty::World { world: trans.unwrap().into() }
    };

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(x_radius, y_radius, 0.0));
    let mut body = ::nphysics::object::RigidBody::new_static(shape, 0.0, 0.0);
    body.set_collision_groups(group);
    body.set_transformation(pos);

    let entity = entities.create();
    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    let (primitive, groups) = ::graphics::Primitive::Plane.instantiate();
    ::component::StaticDraw::add(
        entity,
        primitive,
        groups,
        color.unwrap_or(config.wall_color),
        world_trans,
        static_draws,
        graphics,
    );
}

pub fn create_floor_ceil<'a>(
    z: f32,
    floor: bool,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[super::FLOOR_CEIL_GROUP, super::WALL_GROUP]);

    let pos = ::na::Isometry3::new(::na::Vector3::z() * z, ::na::zero());
    let world_trans = {
        let trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 200.0)
            .to_superset();
        ::graphics::shader::draw1_vs::ty::World { world: trans.unwrap().into() }
    };

    let orientation = if floor { 1f32 } else { -1f32 };
    let shape = ::ncollide::shape::Plane::new(orientation * ::na::Vector3::z());
    let mut body = ::nphysics::object::RigidBody::new_static(shape, 0.0, 0.0);
    body.set_collision_groups(group);
    body.set_transformation(pos);

    let entity = entities.create();

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    let (primitive, groups) = ::graphics::Primitive::Plane.instantiate();
    ::component::StaticDraw::add(
        entity,
        primitive,
        groups,
        config.floor_ceil_color,
        world_trans,
        static_draws,
        graphics,
    );
}
