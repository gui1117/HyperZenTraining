use alga::general::SubsetOf;

const WALL_GROUP: usize = 1;
const FLOOR_CEIL_GROUP: usize = 2;

const DIM2_GROUP: usize = 3;

const ALIVE_GROUP: usize = 4;

// TODO: use usize instead of f32
pub fn create_player<'a>(
    pos: [f32; 2],
    players: &mut ::specs::WriteStorage<'a, ::component::Player>,
    aims: &mut ::specs::WriteStorage<'a, ::component::Aim>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    shooters: &mut ::specs::WriteStorage<'a, ::component::Shooter>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let shape = ::ncollide::shape::Cylinder::new(0.4, 0.1);
    let pos = ::na::Isometry3::new(
        ::na::Vector3::new(pos[0], pos[1], 0.5),
        ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
    );

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[DIM2_GROUP, ALIVE_GROUP]);

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
    shooters.insert(entity, ::component::Shooter::new(2.0));

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
}

// IDEA: mabye make it turn on itself
pub fn create_avoider<'a>(
    pos: [f32; 2],
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    avoiders: &mut ::specs::WriteStorage<'a, ::component::Avoider>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let size = 0.1;

    let mut primitive_trans: ::na::Transform3<f32> = ::na::one();
    primitive_trans[(0, 0)] = size * 2.0;
    primitive_trans[(1, 1)] = size * 2.0;
    primitive_trans[(2, 2)] = size * 2.0;

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
    group.set_membership(&[ALIVE_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();
    let velocity = 2.5;
    let time_to_reach_v_max = 1.0;
    let ang_damping = 0.8;
    let pnt_to_com = ::na::Vector3::z() * size - body.center_of_mass().coords;

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let primitives = vec![
        (
            ::graphics::primitive::SQUARE_PYRAMID_BASE,
            ::graphics::GROUP_COUNTER.next()
        ),
        (
            ::graphics::primitive::SQUARE_PYRAMID_SIDE_1,
            ::graphics::GROUP_COUNTER.next()
        ),
        (
            ::graphics::primitive::SQUARE_PYRAMID_SIDE_2,
            ::graphics::GROUP_COUNTER.next()
        ),
        (
            ::graphics::primitive::SQUARE_PYRAMID_SIDE_3,
            ::graphics::GROUP_COUNTER.next()
        ),
        (
            ::graphics::primitive::SQUARE_PYRAMID_SIDE_4,
            ::graphics::GROUP_COUNTER.next()
        ),
    ];

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

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    ::component::DynamicDraw::add(
        entity,
        primitives,
        ::graphics::color::GREEN,
        primitive_trans,
        dynamic_draws,
    );
}

pub fn create_bouncer<'a>(
    pos: [f32; 2],
    bouncers: &mut ::specs::WriteStorage<'a, ::component::Bouncer>,
    momentums: &mut ::specs::WriteStorage<'a, ::component::Momentum>,
    contactors: &mut ::specs::WriteStorage<'a, ::component::Contactor>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    entities: &::specs::Entities,
) {
    let size = 0.05;

    let mut primitive_trans: ::na::Transform3<f32> = ::na::one();
    primitive_trans[(0, 0)] = size * 2.0;
    primitive_trans[(1, 1)] = size * 2.0;
    primitive_trans[(2, 2)] = size * 2.0;

    let shape = ::ncollide::shape::Ball3::new(size);
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.5), ::na::zero());

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[ALIVE_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    let mass = 1.0 / body.inv_mass();
    let velocity = 2.5;
    let time_to_reach_v_max = 0.05;
    let ang_damping = 0.8;

    body.set_transformation(pos);
    body.set_collision_groups(group);

    let primitives = vec![
        (
            // TODO: motif random
            ::graphics::primitive::SPHERE,
            ::graphics::GROUP_COUNTER.next()
        ),
    ];

    let entity = entities.create();
    bouncers.insert(entity, ::component::Bouncer);
    momentums.insert(entity, {
        let mut momentum =
            ::component::Momentum::new(mass, velocity, time_to_reach_v_max, ang_damping, None);
        momentum.direction = ::na::Vector3::new_random().normalize();
        momentum
    });
    contactors.insert(entity, ::component::Contactor::new());

    ::component::PhysicBody::add(entity, body, bodies, physic_world);
    ::component::DynamicDraw::add(
        entity,
        primitives,
        ::graphics::color::BLUE,
        primitive_trans,
        dynamic_draws,
    );
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
    group.set_blacklist(&[WALL_GROUP, FLOOR_CEIL_GROUP]);

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
    ::component::StaticDraw::add(
        entity,
        ::graphics::primitive::PLANE,
        ::graphics::GROUP_COUNTER.next(),
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
    group.set_membership(&[FLOOR_CEIL_GROUP]);
    group.set_blacklist(&[WALL_GROUP, FLOOR_CEIL_GROUP, DIM2_GROUP]);

    let pos = ::na::Isometry3::new(::na::Vector3::z() * z, ::na::zero());
    let world_trans = {
        let trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 40.0)
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
    ::component::StaticDraw::add(
        entity,
        ::graphics::primitive::PLANE,
        ::graphics::GROUP_COUNTER.next(),
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

    // TODO: refactor
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
                    ::na::Vector3::y() * ::std::f32::consts::FRAC_PI_2,
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
                    ::na::Vector3::y() * ::std::f32::consts::FRAC_PI_2,
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
                    ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
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
                    ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
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
