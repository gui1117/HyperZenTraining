pub fn create_player_w(pos: ::na::Vector3<f32>, hook: bool, world: &::specs::World) {
    create_player(
        pos,
        hook,
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
    );
}

pub fn create_player<'a>(
    pos: ::na::Vector3<f32>,
    hook: bool,
    players: &mut ::specs::WriteStorage<'a, ::component::Player>,
    aims: &mut ::specs::WriteStorage<'a, ::component::Aim>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    shooters: &mut ::specs::WriteStorage<'a, ::component::Shooter>,
    hooks: &mut ::specs::WriteStorage<'a, ::component::Hook>,
    weapon_animations: &mut ::specs::WriteStorage<'a, ::component::WeaponAnimation>,
    weapon_anchors: &mut ::specs::WriteStorage<'a, ::component::WeaponAnchor>,
    dynamic_huds: &mut ::specs::WriteStorage<'a, ::component::DynamicHud>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let shape = ::ncollide::shape::Cylinder::new(::CONFIG.player_height, ::CONFIG.player_radius);

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

    if hook {
        let (hook_primitive, hook_groups) = ::graphics::Primitive::Hook.instantiate();
        let hook_primitive_trans = ::graphics::resizer(
            ::CONFIG.player_hook_size,
            ::CONFIG.player_hook_size,
            ::CONFIG.player_hook_size,
        );
        let hook_draw_entity = entities.create();
        dynamic_graphics_assets.insert(hook_draw_entity,
            ::component::DynamicGraphicsAssets::new(
                    hook_primitive,
                    hook_groups,
                    ::CONFIG.player_hook_color,
                    hook_primitive_trans,
            ));
        hooks.insert(entity, ::component::Hook::new(::CONFIG.player_hook_force, hook_draw_entity));
    }
    let velocity = if hook { ::CONFIG.player_hook_velocity } else { ::CONFIG.player_velocity };
    let time_to_reach_vmax = if hook { ::CONFIG.player_hook_time_to_reach_vmax } else { ::CONFIG.player_time_to_reach_vmax };

    momentums.insert(
        entity,
        ::component::Momentum::new(
            mass,
            velocity,
            time_to_reach_vmax,
            None,
            ::CONFIG.player_ang_damping,
            None,
        ),
    );
    super::create_weapon(
        entity,
        shooters,
        weapon_animations,
        weapon_anchors,
        dynamic_huds,
        dynamic_graphics_assets,
        entities,
    );

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
}
