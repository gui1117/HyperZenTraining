use alga::general::SubsetOf;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

pub const WALL_GROUP:       usize = 1; // Both vertical walls and ceil and floor
pub const FLOOR_CEIL_GROUP: usize = 2;
pub const ALIVE_GROUP:      usize = 3;
pub const MONSTER_GROUP:    usize = 4;

pub fn create_light_ray<'a>(
    from: ::na::Vector3<f32>,
    to: ::na::Vector3<f32>,
    radius: f32,
    deleters: &mut ::specs::WriteStorage<'a, ::component::Deleter>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_huds: &mut ::specs::WriteStorage<'a, ::component::DynamicHud>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    entities: &::specs::Entities,
) {
    let (primitive, groups) = ::graphics::Primitive::Cylinder.instantiate();
    let color = ::graphics::color::YELLOW;
    let primitive_trans = {
        let i = ::na::Translation::from_vector((from + to) / 2.0) *
            ::na::Rotation3::rotation_between(
                &::na::Vector3::new(1.0, 0.0, 0.0),
                &(to - from),
            ).unwrap();

        let r = ::na::Rotation3::new(::na::Vector3::new(0.0, -FRAC_PI_2, 0.0));

        i * r * ::graphics::resizer(radius, radius, (to - from).norm() / 2.0)
    };

    let entity = entities.create();
    dynamic_huds.insert(entity, ::component::DynamicHud);
    dynamic_draws.insert(entity, ::component::DynamicDraw);
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            color,
            primitive_trans,
        ),
    );
    deleters.insert(entity, ::component::Deleter::new(0.03));
}

pub fn create_weapon<'a>(
    anchor: ::specs::Entity,
    shooters: &mut ::specs::WriteStorage<'a, ::component::Shooter>,
    weapon_animations: &mut ::specs::WriteStorage<'a, ::component::WeaponAnimation>,
    weapon_anchors: &mut ::specs::WriteStorage<'a, ::component::WeaponAnchor>,
    dynamic_huds: &mut ::specs::WriteStorage<'a, ::component::DynamicHud>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    entities: &::specs::Entities,
) {
    let coef = 3.0;
    let shoot_pos_x = 0.08*coef;
    let weapon_pos_y = -0.02*coef;
    let weapon_pos_z = -0.016*coef;

    let center_radius = 0.0036*coef;
    let light_ray_radius = 0.002*coef;

    let six_radius = 0.0056*coef;
    let six_length = 0.051*coef;

    let bar_x_pos = 0.071*coef;
    let bar_x_radius = 0.04*coef;
    let bar_y_radius = 0.0022*coef;
    let bar_z_radius = 0.0014*coef;

    let bullet_radius= 0.006*coef;
    let bullet_length = 0.0005*coef;
    let bullet_x = 0.035*coef;
    let bullet_dx = 0.003*coef;
    let bullet_nbr = 5;
    let mut bullets = vec![];

    // Six
    let (primitive, groups) = ::graphics::Primitive::Six.instantiate();
    let color = ::graphics::color::RED;
    let primitive_trans = ::na::Rotation3::new(::na::Vector3::new(0.0, FRAC_PI_2, 0.0)) *
        ::graphics::resizer(six_radius, six_radius, six_length);

    let entity = entities.create();
    weapon_anchors.insert(entity, ::component::WeaponAnchor { anchor: anchor });
    dynamic_huds.insert(entity, ::component::DynamicHud);
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            color,
            primitive_trans,
        ),
    );

    // Bullet
    for i in 0..bullet_nbr {
        let (primitive, groups) = ::graphics::Primitive::Six.instantiate();
        let color = ::graphics::color::PALE_BLUE;
        let primitive_trans = ::na::Isometry3::new(
                ::na::Vector3::new(bullet_x + bullet_dx*i as f32, 0.0, 0.0),
                ::na::Vector3::new(0.0, FRAC_PI_2, 0.0),
            ) *
            ::graphics::resizer(bullet_radius, bullet_radius, bullet_length);

        let entity = entities.create();
        bullets.push(entity);
        weapon_anchors.insert(entity, ::component::WeaponAnchor { anchor: anchor });
        dynamic_huds.insert(entity, ::component::DynamicHud);
        dynamic_graphics_assets.insert(
            entity,
            ::component::DynamicGraphicsAssets::new(
                primitive,
                groups,
                color,
                primitive_trans,
            ),
        );
    }
    bullets.reverse();

    for angle in (0..3usize).map(|i| i as f32 * 2.0 * FRAC_PI_3) {
        // Bar
        let (primitive, groups) = ::graphics::Primitive::Cube.instantiate();
        let color = ::graphics::color::PALE_PURPLE;
        let primitive_trans = ::na::Isometry3::new(
            ::na::Vector3::new(
                bar_x_pos,
                (center_radius + bar_y_radius) * angle.cos(),
                (center_radius + bar_y_radius) * angle.sin(),
            ),
            ::na::Vector3::new(angle, 0.0, 0.0),
        ) *
            ::graphics::resizer(bar_x_radius, bar_y_radius, bar_z_radius);

        let entity = entities.create();
        weapon_anchors.insert(entity, ::component::WeaponAnchor { anchor: anchor });
        dynamic_huds.insert(entity, ::component::DynamicHud);
        dynamic_graphics_assets.insert(
            entity,
            ::component::DynamicGraphicsAssets::new(
                primitive,
                groups,
                color,
                primitive_trans,
            ),
        );
    }

    weapon_animations.insert(
        anchor,
        ::component::WeaponAnimation {
            weapon_trans: ::na::Translation3::new(0.0, weapon_pos_y, weapon_pos_z).to_superset(),
            shoot_pos: ::na::Point3::new(shoot_pos_x, 0.0, 0.0),
            light_ray_radius,
            bullets,
        },
    );
    shooters.insert(anchor, ::component::Shooter::new(0.5, bullet_nbr));
}

pub fn create_player<'a>(
    pos: [f32; 2],
    players: &mut ::specs::WriteStorage<'a, ::component::Player>,
    aims: &mut ::specs::WriteStorage<'a, ::component::Aim>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    shooters: &mut ::specs::WriteStorage<'a, ::component::Shooter>,
    weapon_animations: &mut ::specs::WriteStorage<'a, ::component::WeaponAnimation>,
    weapon_anchors: &mut ::specs::WriteStorage<'a, ::component::WeaponAnchor>,
    dynamic_huds: &mut ::specs::WriteStorage<'a, ::component::DynamicHud>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let shape = ::ncollide::shape::Cylinder::new(0.4, 0.1);
    let pos = ::na::Isometry3::new(
        ::na::Vector3::new(pos[0], pos[1], 0.5),
        ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
    );

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[ALIVE_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    body.set_transformation(pos);
    body.set_collision_groups(group);

    let mass = 1.0 / body.inv_mass();
    let velocity = 10.0;
    let time_to_reach_v_max = 0.1;
    let ang_damping = 0.0;

    let entity = entities.create();
    players.insert(entity, ::component::Player);
    aims.insert(entity, ::component::Aim::new());
    momentums.insert(
        entity,
        ::component::Momentum::new(mass, velocity, time_to_reach_v_max, ang_damping, None),
    );
    create_weapon(
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

// IDEA: mabye make it turn on itself
pub fn create_avoider<'a>(
    pos: [f32; 2],
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
    let size = 0.1;

    let primitive_trans = ::graphics::resizer(size, size, size);

    let shape = {
        let mut points = vec![
            ::na::Point3::new(-1.0, -1.0, -1.0),
            ::na::Point3::new(1.0, -1.0, -1.0),
            ::na::Point3::new(1.0, 1.0, -1.0),
            ::na::Point3::new(-1.0, 1.0, -1.0),
            ::na::Point3::new(0.0, 0.0, 1.0),
        ];
        for p in &mut points {
            *p = *p * size
        }
        ::ncollide::shape::ConvexHull::new(points)
    };

    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.5), ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[ALIVE_GROUP, MONSTER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();
    let velocity = 5.5;
    let time_to_reach_v_max = 1.0;
    let ang_damping = 0.8;
    let pnt_to_com = ::na::Vector3::z() * size - body.center_of_mass().coords;

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let (primitive, groups) = ::graphics::Primitive::SquarePyramid.instantiate();
    let color = ::graphics::color::GREEN;

    let entity = entities.create();
    avoiders.insert(entity, ::component::Avoider::new());
    momentums.insert(
        entity,
        ::component::Momentum::new(
            mass,
            velocity,
            time_to_reach_v_max,
            ang_damping,
            Some(pnt_to_com),
        ),
    );
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

pub fn create_bouncer<'a>(
    pos: [f32; 2],
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
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.5), ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[ALIVE_GROUP, MONSTER_GROUP]);

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

pub fn create_turret<'a>(
    pos: [f32; 2],
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    turrets: &mut ::specs::WriteStorage<'a, ::component::Turret>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    lifes: &mut ::specs::WriteStorage<'a, ::component::Life>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let size = 0.15;

    let primitive_trans = ::graphics::resizer(size, size, size);

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(size, size, size));
    let pos = ::na::Point3::new(pos[0], pos[1], 0.5);
    let trans = ::na::Isometry3::new(pos.coords, ::na::Vector3::new(0.0, FRAC_PI_2, 0.0));

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[ALIVE_GROUP, MONSTER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 10.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();
    let velocity = 0.01;
    let time_to_reach_v_max = 0.05;
    let ang_damping = 0.1;

    body.set_transformation(trans);
    body.set_collision_groups(group);

    // Create laser
    let (laser_primitive, laser_groups) = ::graphics::Primitive::Cylinder.instantiate();
    let laser_color = ::graphics::color::BLACK;
    let laser_entity = entities.create();
    dynamic_graphics_assets.insert(
        laser_entity,
        ::component::DynamicGraphicsAssets::new(
            laser_primitive,
            laser_groups,
            laser_color,
            ::na::one(),
        ),
    );
    dynamic_draws.insert(laser_entity, ::component::DynamicDraw);

    // Create turret
    let (primitive, groups) = ::graphics::Primitive::PitCube.instantiate();
    let color = ::graphics::color::PURPLE;

    let entity = entities.create();
    turrets.insert(entity, ::component::Turret {
        laser: laser_entity,
    });
    momentums.insert(entity, ::component::Momentum::new(mass, velocity, time_to_reach_v_max, ang_damping, Some(::na::Vector3::new(0.0, 0.0, 1.0))));
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            color,
            primitive_trans,
        ),
    );
    lifes.insert(entity, ::component::Life::DrawAlive);
    dynamic_draws.insert(entity, ::component::DynamicDraw);

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    bodies.get_mut(entity).unwrap().ball_in_socket(physic_world, ::na::Point3::new(pos[0], pos[1], 0.5));
}

pub fn create_wall_side<'a>(
    pos: ::na::Isometry3<f32>,
    x_radius: f32,
    y_radius: f32,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    entities: &::specs::Entities,
) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[WALL_GROUP]);

    let world_trans = {
        let pos_trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 1.0)
            .to_superset();
        let mut dim_trans: ::na::Transform3<f32> = ::na::one();
        dim_trans[(0, 0)] = x_radius;
        dim_trans[(1, 1)] = y_radius;
        let trans = pos_trans * dim_trans;
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
        ::graphics::color::PALE_RED,
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
    entities: &::specs::Entities,
) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[FLOOR_CEIL_GROUP, WALL_GROUP]);

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
        ::graphics::color::PALE_BROWN,
        world_trans,
        static_draws,
        graphics,
    );
}

pub fn create_maze_walls<'a>(
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    maze: &::specs::Fetch<'a, ::resource::Maze>,
    entities: &::specs::Entities,
) {
    create_floor_ceil(
        0.0,
        true,
        bodies,
        static_draws,
        physic_world,
        graphics,
        entities,
    );
    create_floor_ceil(
        1.0,
        false,
        bodies,
        static_draws,
        physic_world,
        graphics,
        entities,
    );

    let size = {
        assert_eq!(maze.height, maze.width);
        maze.height
    };
    let maze = &maze.walls;

    for x in 0..size {
        let mut up_coords = None;
        let mut down_coords = None;
        for y in 0..size + 1 {
            let up_wall = if x == 0 || y == size {
                false
            } else {
                maze[x - 1][y]
            };
            let wall = if y == size { false } else { maze[x][y] };
            let down_wall = if x + 1 == size || y == size {
                false
            } else {
                maze[x + 1][y]
            };

            let up_side_wall = wall && !up_wall;
            let down_side_wall = wall && !down_wall;

            if up_side_wall && up_coords.is_none() {
                up_coords = Some((y, y));
            } else if up_side_wall && up_coords.is_some() {
                up_coords.as_mut().unwrap().1 = y;
            } else if !up_side_wall && up_coords.is_some() {
                let c = up_coords.take().unwrap();
                let x_radius = 0.5;
                let y_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(x as f32, c.0 as f32 + y_radius, 0.5),
                    ::na::Vector3::y() * FRAC_PI_2,
                );
                create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }

            if down_side_wall && down_coords.is_none() {
                down_coords = Some((y, y));
            } else if down_side_wall && down_coords.is_some() {
                down_coords.as_mut().unwrap().1 = y;
            } else if !down_side_wall && down_coords.is_some() {
                let c = down_coords.take().unwrap();
                let x_radius = 0.5;
                let y_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(x as f32 + 1.0, c.0 as f32 + y_radius, 0.5),
                    ::na::Vector3::y() * FRAC_PI_2,
                );
                create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }
        }
    }

    for y in 0..size {
        let mut up_coords = None;
        let mut down_coords = None;
        for x in 0..size + 1 {
            let up_wall = if y == 0 || x == size {
                false
            } else {
                maze[x][y - 1]
            };
            let wall = if x == size { false } else { maze[x][y] };
            let down_wall = if y + 1 == size || x == size {
                false
            } else {
                maze[x][y + 1]
            };

            let up_side_wall = wall && !up_wall;
            let down_side_wall = wall && !down_wall;

            if up_side_wall && up_coords.is_none() {
                up_coords = Some((x, x));
            } else if up_side_wall && up_coords.is_some() {
                up_coords.as_mut().unwrap().1 = x;
            } else if !up_side_wall && up_coords.is_some() {
                let c = up_coords.take().unwrap();
                let x_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let y_radius = 0.5;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(c.0 as f32 + x_radius, y as f32, 0.5),
                    ::na::Vector3::x() * FRAC_PI_2,
                );
                create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }

            if down_side_wall && down_coords.is_none() {
                down_coords = Some((x, x));
            } else if down_side_wall && down_coords.is_some() {
                down_coords.as_mut().unwrap().1 = x;
            } else if !down_side_wall && down_coords.is_some() {
                let c = down_coords.take().unwrap();
                let x_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let y_radius = 0.5;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(c.0 as f32 + x_radius, y as f32 + 1.0, 0.5),
                    ::na::Vector3::x() * FRAC_PI_2,
                );
                create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }
        }
    }
}
