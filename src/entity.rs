use alga::general::SubsetOf;

const WALL_GROUP: usize = 1;
const FLOOR_CEIL_GROUP: usize = 2;

const DIM2_GROUP: usize = 3;

const ALIVE_GROUP: usize = 4;

// TODO: use usize instead of f32
pub fn create_player(world: &mut ::specs::World, pos: [f32; 2]) {
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

    let bodyhandle = world
        .write_resource::<::resource::PhysicWorld>()
        .add_rigid_body(body);

    // TODO: ccd ?
    // world.write_resource::<::resource::PhysicWorld>().0.add_ccd_to(&bodyhandle, 0.01, false);
    let entity = world
        .create_entity()
        .with(::component::Player)
        .with(::component::Aim::new())
        .with(::component::Momentum::new(
            mass,
            velocity,
            time_to_reach_v_max,
            ang_damping,
            None,
        ))
        .with(::component::Shooter::new(2.0))
        .build();

    ::component::PhysicBody::add(world, entity, bodyhandle);
}

// IDEA: maybe have a triangle base for the pyramid
// IDEA: mabye make it turn on itself
pub fn create_avoider(world: &mut ::specs::World, pos: [f32; 2]) {
    let size = 0.1;

    let mut primitive_trans: ::na::Transform3<f32> = ::na::one();
    primitive_trans[(0, 0)] = size;
    primitive_trans[(1, 1)] = size;
    primitive_trans[(2, 2)] = size;

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
    let velocity = 5.0;
    let time_to_reach_v_max = 1.0;
    let ang_damping = 0.8;
    let pnt_to_com = ::na::Vector3::z() * size - body.center_of_mass().coords;

    body.set_transformation(pos);
    body.set_collision_groups(group);
    let bodyhandle = world
        .write_resource::<::resource::PhysicWorld>()
        .add_rigid_body(body);
    let entity = world
        .create_entity()
        .with(::component::Avoider::new())
        .with(::component::Momentum::new(
            mass,
            velocity,
            time_to_reach_v_max,
            ang_damping,
            Some(pnt_to_com),
        ))
        .build();

    // IDEA: same graphics group for all avoider ?
    ::component::DynamicDraw::add(world, entity, ::graphics::GROUP_COUNTER.next(), primitive_trans);
    ::component::PhysicBody::add(world, entity, bodyhandle);
}

pub fn create_wall_side(
    world: &mut ::specs::World,
    pos: ::na::Isometry3<f32>,
    x_radius: f32,
    y_radius: f32,
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
        ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
    };

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(x_radius, y_radius, 0.0));
    let mut body = ::nphysics::object::RigidBody::new_static(shape, 0.0, 0.0);
    body.set_collision_groups(group);
    body.set_transformation(pos);
    let bodyhandle = world
        .write_resource::<::resource::PhysicWorld>()
        .add_rigid_body(body);

    let entity = world.create_entity().build();
    ::component::StaticDraw::add(world, entity, ::graphics::GROUP_COUNTER.next(), world_trans);
    ::component::PhysicBody::add(world, entity, bodyhandle);
}

pub fn create_floor_ceil(world: &mut ::specs::World, z: f32, floor: bool) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[FLOOR_CEIL_GROUP]);
    group.set_blacklist(&[WALL_GROUP, FLOOR_CEIL_GROUP, DIM2_GROUP]);

    let pos = ::na::Isometry3::new(::na::Vector3::z()*z, ::na::zero());
    let world_trans = {
        let trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 40.0).to_superset();
        ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
    };

    let orientation = if floor { 1f32 } else { -1f32 };
    let shape = ::ncollide::shape::Plane::new(orientation*::na::Vector3::z());
    let mut body = ::nphysics::object::RigidBody::new_static(shape, 0.0, 0.0);
    body.set_collision_groups(group);
    body.set_transformation(pos);
    let bodyhandle = world.write_resource::<::resource::PhysicWorld>().add_rigid_body(body);

    let entity = world.create_entity().build();
    ::component::StaticDraw::add(world, entity, ::graphics::GROUP_COUNTER.next(), world_trans);
    ::component::PhysicBody::add(world, entity, bodyhandle);
}

pub fn create_maze_walls(world: &mut ::specs::World) {
    // TODO: do not clone maze.
    //       maybe a method instantiate on maze that take world
    //       or all entity method take storage instead of whole world
    let maze = world.read_resource::<::resource::Maze>().clone();

    create_floor_ceil(world, 0.0, true);
    create_floor_ceil(world, 1.0, false);

    // TODO: refactor
    let size = {
        assert_eq!(maze.height, maze.width);
        maze.height
    };
    let maze = maze.walls;

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
                create_wall_side(world, pos, x_radius, y_radius);
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
                create_wall_side(world, pos, x_radius, y_radius);
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
                create_wall_side(world, pos, x_radius, y_radius);
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
                create_wall_side(world, pos, x_radius, y_radius);
            }
        }
    }
}
