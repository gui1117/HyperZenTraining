pub fn create_player_w(pos: ::na::Vector3<f32>, world: &::specs::World) {
    create_player(
        pos,
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
        &mut world.write(),
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
        &world.read_resource(),
    );
}

pub fn create_player<'a>(
    pos: ::na::Vector3<f32>,
    players: &mut ::specs::WriteStorage<'a, ::component::Player>,
    aims: &mut ::specs::WriteStorage<'a, ::component::Aim>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    air_momentums: &mut ::specs::WriteStorage<'a, ::component::AirMomentum>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    shooters: &mut ::specs::WriteStorage<'a, ::component::Shooter>,
    hooks: &mut ::specs::WriteStorage<'a, ::component::Hook>,
    weapon_animations: &mut ::specs::WriteStorage<'a, ::component::WeaponAnimation>,
    weapon_anchors: &mut ::specs::WriteStorage<'a, ::component::WeaponAnchor>,
    dynamic_huds: &mut ::specs::WriteStorage<'a, ::component::DynamicHud>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    contactors: &mut ::specs::WriteStorage<'a, ::component::Contactor>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let shape = ::ncollide::shape::Cylinder::new(config.player_height, config.player_radius);

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[super::ALIVE_GROUP, super::PLAYER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let pos = ::na::Isometry3::new(pos, ::na::zero());
    body.set_transformation(pos);
    body.set_collision_groups(group);

    let mass = 1.0 / body.inv_mass();

    let entity = entities.create();
    players.insert(entity, ::component::Player);
    aims.insert(entity, ::component::Aim::new());
    contactors.insert(entity, ::component::Contactor::new());
    hooks.insert(entity, ::component::Hook::new(config.player_hook_force));
    momentums.insert(
        entity,
        ::component::Momentum::new(
            mass,
            config.player_velocity,
            config.player_time_to_reach_vmax,
            None,
            config.player_ang_damping,
            None,
        ),
    );
    air_momentums.insert(
        entity,
        ::component::AirMomentum {
            gravity_force: config.player_gravity,
            damping: config.player_air_damping,
        },
    );
    super::create_weapon(
        entity,
        shooters,
        weapon_animations,
        weapon_anchors,
        dynamic_huds,
        dynamic_graphics_assets,
        config,
        entities,
    );

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
}
