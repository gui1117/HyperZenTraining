use ncollide::world::CollisionGroups;
use alga::general::SubsetOf;
use itertools::Itertools;
use std::collections::{HashMap};

const PLAYER_GROUP: usize = 0;
const WALL_GROUP: usize = 1;

pub fn create_player(world: &mut ::specs::World, pos: [f32; 2]) {
    let shape = ::ncollide::shape::Cylinder::new(0.5, 0.1);
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.0), ::na::Vector3::x()*::std::f32::consts::FRAC_PI_2);

    let mut group = CollisionGroups::new();
    group.set_membership(&[PLAYER_GROUP]);

    let mut body = ::nphysics::object::RigidBody::new_dynamic(shape, 1.0, 1.0, 1.0);
    body.set_transformation(pos);
    let mass = 1.0 / body.inv_mass();
    let velocity = 10.0;
    let time_to_reach_v_max = 0.1;

    let bodyhandle = world.write_resource::<::resource::PhysicWorld>().0.add_rigid_body(body);
    let entity = world.create_entity()
        .with(::component::Player)
        .with(::component::PhysicRigidBodyHandle::new(bodyhandle))
        .with(::component::Momentum::new(mass, velocity, time_to_reach_v_max))
        .build();
    // ::component::DynamicDraw::add(world, entity, 2);
}

pub fn create_wall(world: &mut ::specs::World, pos: [f32; 2], x_radius: f32, y_radius: f32) {
    let mut group = CollisionGroups::new();
    group.set_membership(&[WALL_GROUP]);
    group.set_blacklist(&[WALL_GROUP]);

    let shape = ::ncollide::shape::Cuboid::new(::na::Vector3::new(x_radius, y_radius, 0.5));
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.0), ::na::zero());

    let world_trans = {
        let mut trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 0.5)
            .to_superset();
        // TODO this is not very legit
        trans[(0, 0)] = x_radius;
        trans[(1, 1)] = y_radius;
        ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
    };

    let body = ::nphysics::object::RigidBody::new_static(shape, 1.0, 1.0);
    let bodyhandle = world.write_resource::<::resource::PhysicWorld>().0.add_rigid_body(body);
    bodyhandle.borrow_mut().set_transformation(pos);

    let entity = world.create_entity()
        .with(::component::PhysicRigidBodyHandle::new(bodyhandle))
        .build();
    ::component::StaticDraw::add(world, entity, 1, world_trans);
}

pub fn create_maze_walls(world: &mut ::specs::World, maze: Vec<Vec<bool>>) {
    let (mut x_map, mut y_map) = maze.iter()
        .enumerate()
        .flat_map(|(x, column)| {
            column.iter()
                .enumerate()
                .filter_map(move |(y, &wall)| {
                    if wall {
                        Some((x, y))
                    } else {
                        None
                    }
                })
        })
        .fold((HashMap::new(), HashMap::new()), |(mut x_map, mut y_map), (x, y)| {
            x_map.entry(x).or_insert(vec!()).push(y);
            y_map.entry(y).or_insert(vec!()).push(x);
            (x_map, y_map)
        });

    x_map.values_mut().foreach(|vec| vec.sort());
    y_map.values_mut().foreach(|vec| vec.sort());

    let flat_mapper = |(x, ys): (&usize, &Vec<usize>)| -> Vec<(usize, usize, usize)> {
        let mut walls = vec!();

        let mut start = *ys.first().unwrap();
        let mut end = start;
        for &y in ys {
            if y == end + 1 {
                end = y;
            } else {
                walls.push((*x, start, end));
                start = y;
                end = y;
            }
        }
        walls.push((*x, start, end));
        walls
    };

    x_map
        .iter()
        .flat_map(|elt| flat_mapper(elt))
        .foreach(|(x, y_start, y_end)| {
            let x_radius = 0.5;
            let y_radius = (y_end - y_start + 1) as f32 / 2.0;

            let x = x as f32;
            let y = y_start as f32 + (y_end - y_start) as f32/2.0;

            create_wall(world, [x, y], x_radius, y_radius);
        });

    y_map
        .iter()
        .flat_map(|elt| flat_mapper(elt))
        .foreach(|(y, x_start, x_end)| {
            let x_radius = (x_end - x_start + 1) as f32 / 2.0;
            let y_radius = 0.5;

            let x = x_start as f32 + (x_end - x_start) as f32/2.0;
            let y = y as f32;

            // create_wall(world, [x, y], x_radius, y_radius);
        });
}

// TODO ceil floor
//    // let floor = ShapeHandle3::new(Plane::new(na::Vector3::new(0.0, 0.0, 1.0)));
//    // world.deferred_add(0, na::Isometry3::identity(), floor, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

    // let mut plane_transform = na::Transform3::identity();
    // plane_transform[(0, 0)] = 10.;
    // plane_transform[(1, 1)] = 10.;
    // let floor_world_trans = plane_transform *
    //     na::Translation3::from_vector([0.0, 0.0, -10.5].into());

    // let floor_uniform_buffer =
    //     vulkano::buffer::cpu_access::CpuAccessibleBuffer::<graphics::shader::vs::ty::World>::from_data(
    //         graphics.data.device.clone(),
    //         vulkano::buffer::BufferUsage::uniform_buffer(),
    //         graphics::shader::vs::ty::World { world: floor_world_trans.unwrap().into() },
    //     ).expect("failed to create buffer");

    // let floor_set = Arc::new(
    //     vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
    //         graphics.data.pipeline.clone(),
    //         0,
    //     ).add_buffer(floor_uniform_buffer.clone())
    //         .unwrap()
    //         .build()
    //         .unwrap(),
    // );

    // let ceil_world_trans = plane_transform * na::Translation3::from_vector([0.0, 0.0, 1.5].into());

    // let ceil_uniform_buffer =
    //     vulkano::buffer::cpu_access::CpuAccessibleBuffer::<graphics::shader::vs::ty::World>::from_data(
    //         graphics.data.device.clone(),
    //         vulkano::buffer::BufferUsage::uniform_buffer(),
    //         graphics::shader::vs::ty::World { world: ceil_world_trans.unwrap().into() },
    //     ).expect("failed to create buffer");

    // let ceil_set = Arc::new(
    //     vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
    //         graphics.data.pipeline.clone(),
    //         0,
    //     ).add_buffer(ceil_uniform_buffer.clone())
    //         .unwrap()
    //         .build()
    //         .unwrap(),
    // );

