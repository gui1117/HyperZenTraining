pub fn create_teleport_w(pos: ::na::Isometry3<f32>, scale: f32, action: ::resource::LevelAction, world: &::specs::World) {
    create_teleport(
        pos,
        scale,
        action,
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write_resource(),
        &world.read_resource(),
    );
}

pub fn create_teleport<'a>(
    pos: ::na::Isometry3<f32>,
    scale: f32,
    action: ::resource::LevelAction,
    teleports: &mut ::specs::WriteStorage<'a, ::component::Teleport>,
    proximitors: &mut ::specs::WriteStorage<'a, ::component::Proximitor>,
    sensors: &mut ::specs::WriteStorage<'a, ::component::PhysicSensor>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(0.45 * scale, 0.45 * scale, 0.0));
    let pos =
        pos * ::na::Translation3::from_vector(::na::Vector3::new(0.0, 0.0, ::CONFIG.teleport_dl));

    let mut group = ::nphysics::object::SensorCollisionGroups::new();
    group.set_whitelist(&[super::PLAYER_GROUP]);
    group.set_membership(&[super::TELEPORT_GROUP]);

    let mut sensor = ::nphysics::object::Sensor::new(shape, None);
    sensor.set_relative_position(pos);
    sensor.set_collision_groups(group);

    let entity = entities.create();
    proximitors.insert(entity, ::component::Proximitor::new());
    teleports.insert(entity, ::component::Teleport {
        action,
    });
    ::component::PhysicSensor::add(entity, sensor, sensors, physic_world);
}
