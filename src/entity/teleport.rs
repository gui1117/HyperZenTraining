use alga::general::SubsetOf;

pub fn create_teleport<'a>(
    pos: ::na::Isometry3<f32>,
    teleports: &mut ::specs::WriteStorage<'a, ::component::Teleport>,
    proximitors: &mut ::specs::WriteStorage<'a, ::component::Proximitor>,
    sensors: &mut ::specs::WriteStorage<'a, ::component::PhysicSensor>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    entities: &::specs::Entities,
) {
    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(0.4, 0.4, 0.0));
    let pos = pos * ::na::Translation3::from_vector(::na::Vector3::new(0.0, 0.0, - 0.4));
    let world_trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 0.4).to_superset();

    let mut group = ::nphysics::object::SensorCollisionGroups::new();
    group.set_whitelist(&[super::PLAYER_GROUP]);

    let mut sensor = ::nphysics::object::Sensor::new(shape, None);
    sensor.set_relative_position(pos);
    sensor.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::Plane.instantiate();
    let color = ::graphics::color::GREEN;

    let entity = entities.create();
    proximitors.insert(entity, ::component::Proximitor::new());
    teleports.insert(entity, ::component::Teleport);
    ::component::PhysicSensor::add(entity, sensor, sensors, physic_world);
    ::component::StaticDraw::add(
        entity,
        primitive,
        groups,
        color,
        ::graphics::shader::draw1_vs::ty::World {
            world: world_trans.unwrap().into()
        },
        static_draws,
        graphics,
    );
}
