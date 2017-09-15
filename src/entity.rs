use alga::general::SubsetOf;

const PLAYER_GROUP: usize = 0;
const WALL_GROUP: usize = 1;
const AVOIDER_GROUP: usize = 2;

pub fn create_player(world: &mut ::specs::World, pos: [f32; 2]) {
    let shape = ::ncollide::shape::Cylinder::new(0.5, 0.1);
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.5), ::na::Vector3::x()*::std::f32::consts::FRAC_PI_2);

    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_dynamic();
    group.set_membership(&[PLAYER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    body.set_transformation(pos);
    body.set_collision_groups(group);
    let mass = 1.0 / body.inv_mass();
    let velocity = 10.0;
    let time_to_reach_v_max = 0.1;

    let bodyhandle = world.write_resource::<::resource::PhysicWorld>().0.add_rigid_body(body);
    world.write_resource::<::resource::PhysicWorld>().0.add_ccd_to(&bodyhandle, 0.01, false);
    world.create_entity()
        .with(::component::Player)
        .with(::component::PhysicRigidBodyHandle::new(bodyhandle))
        .with(::component::Momentum::new(mass, velocity, time_to_reach_v_max))
        .build();
}

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
    group.set_membership(&[AVOIDER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 0.0, 0.0);
    body.set_transformation(pos);
    body.set_collision_groups(group);
    let mass = 1.0 / body.inv_mass();
    let velocity = 5.0;
    let time_to_reach_v_max = 1.0;

    let bodyhandle = world.write_resource::<::resource::PhysicWorld>().0.add_rigid_body(body);
    world.write_resource::<::resource::PhysicWorld>().0.add_ccd_to(&bodyhandle, 0.01, false);
    let entity = world.create_entity()
        .with(::component::PhysicRigidBodyHandle::new(bodyhandle))
        .with(::component::Momentum::new(mass, velocity, time_to_reach_v_max))
        .build();
    // TODO same graphics group for all avoider ?
    ::component::DynamicDraw::add(world, entity, ::graphics::GROUP_COUNTER.next(), primitive_trans);
}

pub fn create_wall_side(world: &mut ::specs::World, pos: ::na::Isometry3<f32>, x_radius: f32, y_radius: f32) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[WALL_GROUP]);
    group.set_blacklist(&[WALL_GROUP]);

    let world_trans = {
        let pos_trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 1.0)
            .to_superset();
        let mut dim_trans: ::na::Transform3<f32> = ::na::one();
        dim_trans[(0, 0)] = x_radius;
        dim_trans[(1, 1)] = y_radius;
        let trans = pos_trans*dim_trans;
        ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
    };

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(x_radius, y_radius, 0.0));
    let mut body = ::nphysics::object::RigidBody::new_static(shape, 0.0, 0.0);
    body.set_collision_groups(group);
    body.set_transformation(pos);
    let bodyhandle = world.write_resource::<::resource::PhysicWorld>().0.add_rigid_body(body);

    let entity = world.create_entity()
        .with(::component::PhysicRigidBodyHandle::new(bodyhandle))
        .build();
    ::component::StaticDraw::add(world, entity, ::graphics::GROUP_COUNTER.next(), world_trans);
}

pub fn create_maze_walls(world: &mut ::specs::World, maze: Vec<Vec<bool>>) {
    // TODO: refactor
    let size = {
        assert_eq!(maze.len().pow(2), maze.iter().map(|column| column.len()).sum());
        maze.len()
    };

    for x in 0..size {
        let mut up_coords = None;
        let mut down_coords = None;
        for y in 0..size+1 {
            let up_wall = if x == 0 || y == size {
                false
            } else {
                maze[x-1][y]
            };
            let wall = if y == size {
                false
            } else {
                maze[x][y]
            };
            let down_wall = if x + 1 == size || y == size {
                false
            } else {
                maze[x+1][y]
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
                let pos = ::na::Isometry3::new(::na::Vector3::new(x as f32 - 0.5, c.0 as f32 + y_radius - 0.5, 0.5), ::na::Vector3::y()*::std::f32::consts::FRAC_PI_2);
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
                let pos = ::na::Isometry3::new(::na::Vector3::new(x as f32 + 0.5, c.0 as f32 + y_radius - 0.5, 0.5), ::na::Vector3::y()*::std::f32::consts::FRAC_PI_2);
                create_wall_side(world, pos, x_radius, y_radius);
            }
        }
    }

    for y in 0..size {
        let mut up_coords = None;
        let mut down_coords = None;
        for x in 0..size+1 {
            let up_wall = if y == 0 || x == size {
                false
            } else {
                maze[x][y-1]
            };
            let wall = if x == size {
                false
            } else {
                maze[x][y]
            };
            let down_wall = if y + 1 == size || x == size {
                false
            } else {
                maze[x][y+1]
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
                let pos = ::na::Isometry3::new(::na::Vector3::new(c.0 as f32 + x_radius - 0.5, y as f32 - 0.5, 0.5), ::na::Vector3::x()*::std::f32::consts::FRAC_PI_2);
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
                let pos = ::na::Isometry3::new(::na::Vector3::new(c.0 as f32 + x_radius - 0.5, y as f32 + 0.5, 0.5), ::na::Vector3::x()*::std::f32::consts::FRAC_PI_2);
                create_wall_side(world, pos, x_radius, y_radius);
            }
        }
    }
}
