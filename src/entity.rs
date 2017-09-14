use alga::general::SubsetOf;
use itertools::Itertools;
use std::collections::{HashMap};

const PLAYER_GROUP: usize = 0;
const WALL_GROUP: usize = 1;

pub fn create_player(world: &mut ::specs::World, pos: [f32; 2]) {
    let shape = ::ncollide::shape::Cylinder::new(0.5, 0.5);
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
    let entity = world.create_entity()
        .with(::component::Player)
        .with(::component::PhysicRigidBodyHandle::new(bodyhandle))
        .with(::component::Momentum::new(mass, velocity, time_to_reach_v_max))
        .build();
    // ::component::DynamicDraw::add(world, entity, 2);
}

pub fn create_wall_side(world: &mut ::specs::World, pos: ::na::Isometry3<f32>, x_radius: f32, y_radius: f32) {
    let mut group = ::nphysics::object::RigidBodyCollisionGroups::new_static();
    group.set_membership(&[WALL_GROUP]);
    group.set_blacklist(&[WALL_GROUP]);

    let world_trans = {
        let mut pos_trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 1.0)
            .to_superset();
        let mut dim_trans: ::na::Transform3<f32> = ::na::one();
        // TODO this is not very legit
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
    ::component::StaticDraw::add(world, entity, 1, world_trans);
}

pub fn create_maze_walls(world: &mut ::specs::World, maze: Vec<Vec<bool>>) {
    let size = {
        assert_eq!(maze.len().pow(2), maze.iter().map(|column| column.len()).sum());
        maze.len()
    };

    for x in 0..size+1 {
        let mut coords = None;
        for y in 0..size {
            let up_wall = if x == 0 {
                false
            } else {
                maze[x-1][y]
            };
            let wall = if x == size {
                false
            } else {
                maze[x][y]
            };
            let side_wall = up_wall && wall;

            if side_wall && coords.is_none() {
                coords = Some((y, y));
            } else if side_wall && coords.is_some() {
                coords.as_mut().unwrap().1 = y;
            } else if !side_wall && coords.is_some() {
                let c = coords.take().unwrap();
                let x_radius = 0.5;
                let y_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let pos = ::na::Isometry3::new(::na::Vector3::new(x as f32, c.0 as f32 + y_radius, 0.5), ::na::Vector3::y()*::std::f32::consts::FRAC_PI_2);
                create_wall_side(world, pos, x_radius, y_radius);
            }
        }
    }

    for y in 0..size+1 {
        let mut coords = None;
        for x in 0..size {
            let up_wall = if y == 0 {
                false
            } else {
                maze[x][y-1]
            };
            let wall = if y == size {
                false
            } else {
                maze[x][y]
            };
            let side_wall = up_wall && wall;

            if side_wall && coords.is_none() {
                coords = Some((x, x));
            } else if side_wall && coords.is_some() {
                coords.as_mut().unwrap().1 = y;
            } else if !side_wall && coords.is_some() {
                let c = coords.take().unwrap();
                let x_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let y_radius = 0.5;
                let pos = ::na::Isometry3::new(::na::Vector3::new(c.0 as f32 + x_radius, y as f32, 0.5), ::na::Vector3::z()*::std::f32::consts::FRAC_PI_2);
                create_wall_side(world, pos, x_radius, y_radius);
            }
        }
    }
}
