use ncollide::shape::{Cylinder, Cuboid, ShapeHandle3};
use ncollide::world::CollisionGroups;
use alga::general::SubsetOf;
use itertools::Itertools;

const PLAYER_GROUP: usize = 0;
const WALL_GROUP: usize = 1;

pub fn create_player(world: &mut ::specs::World, pos: [f32; 2]) {
    // let shape = ShapeHandle3::new(Ball::new(0.1));
    // let shape = ShapeHandle3::new(Cuboid::new(::na::Vector3::new(0.1, 0.1, 0.1)));
    let shape = ShapeHandle3::new(Cylinder::new(2.5, 0.1));
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.0), ::na::Vector3::x()*::std::f32::consts::FRAC_PI_2);

    let mut group = CollisionGroups::new();
    group.set_membership(&[PLAYER_GROUP]);

    let entity = world.create_entity()
        .with(::component::Player)
        .with(::component::Momentum::new(5.0, 0.1))
        .build();
    ::component::ColBody::add(world, entity, pos, shape, group);
    // ::component::DynamicDraw::add(world, entity, 2);
}

pub fn create_wall(world: &mut ::specs::World, pos: [f32; 2], x_radius: f32, y_radius: f32) {
    let mut group = CollisionGroups::new();
    group.set_membership(&[WALL_GROUP]);
    group.set_blacklist(&[WALL_GROUP]);

    let shape = ShapeHandle3::new(Cuboid::new(::na::Vector3::new(x_radius, y_radius, 0.5)));
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.0), ::na::zero());

    let world_trans = {
        let mut trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 0.5)
            .to_superset();
        // TODO this is not very legit
        trans[(0, 0)] = x_radius;
        trans[(1, 1)] = y_radius;
        ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
    };

    let entity = world.create_entity().build();

    ::component::ColBody::add(world, entity, pos, shape, group);
    ::component::StaticDraw::add(world, entity, 1, world_trans);
}

pub fn create_maze_walls(world: &mut ::specs::World, maze: Vec<Vec<bool>>) {
    let mut group = CollisionGroups::new();
    group.set_membership(&[WALL_GROUP]);
    group.set_blacklist(&[WALL_GROUP]);

    let maze_size = maze.len();

    let iterator = maze.iter()
        .enumerate()
        .flat_map(|(x, column)| {
            column.iter()
                .enumerate()
                .map(move |(y, wall)| {
                    (x, y, wall)
                })
        });

    iterator
        .chunks(maze_size)
        .into_iter()
        .foreach(|chunk| {
            chunk
                .batching(|it| {
                    let mut start = None;
                    loop {
                        match it.next() {
                            Some((x, y, &true)) => {
                                start = Some((x, y));
                                break;
                            },
                            Some((_, _, &false)) => (),
                            None => break,
                        }
                    }
                    if let Some((x, y_start)) = start {
                        let mut y_end = y_start;
                        while let Some((x_indice, y_indice, &true)) = it.next() {
                            assert_eq!(x, x_indice);
                            y_end = y_indice;
                        }
                        Some((x, y_start, y_end))
                    } else {
                        None
                    }
                })
                .foreach(|(x, y_start, y_end)| {
                    let x_radius = 0.5;
                    let y_radius = (y_end - y_start + 1) as f32 / 2.0;

                    let x = x as f32;
                    let y = y_start as f32 + (y_end - y_start) as f32/2.0;

                    create_wall(world, [x, y], x_radius, y_radius);
                })
        });


    // for (x, column) in maze.iter().enumerate() {
    //     for (start, end) in column.iter().enumerate().batching(|it| {
    //         let mut start = None;
    //         loop {
    //             match it.next() {
    //                 Some((indice, &true)) => {
    //                     start = Some(indice);
    //                     break;
    //                 },
    //                 Some((_, &false)) => (),
    //                 None => break,
    //             }
    //         }
    //         if let Some(start) = start {
    //             let mut end = start;
    //             while let Some((indice, &true)) = it.next() {
    //                 end = indice;
    //             }
    //             Some((start as f32, end as f32))
    //         } else {
    //             None
    //         }
    //     }) {
    //         let x_radius = 0.5;
    //         let y_radius = (end - start + 1.0) / 2.0;

    //         let x = x as f32;
    //         let y = start + (end - start)/2.0;

    //         create_wall(world, [x, y], x_radius, y_radius);
    //     }
    // }
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

