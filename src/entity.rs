use ncollide::shape::{Cylinder, Cuboid, ShapeHandle3};
use ncollide::world::CollisionGroups;
use alga::general::SubsetOf;

pub fn create_player(world: &mut ::specs::World) {
    let shape = ShapeHandle3::new(Cylinder::new(0.5f32, 0.3));
    let pos = ::na::Isometry3::new(::na::Vector3::new(-1.0, 0.0, 0.0), ::na::Vector3::z());
    let group = CollisionGroups::new();

    let entity = world.create_entity()
        .with(::component::Player)
        .with(::component::Momentum::new(1.0, 0.1))
        .build();
    ::component::ColBody::add(world, entity, pos, shape, group);
}

pub fn create_wall(world: &mut ::specs::World, pos: [f32; 2]) {
    let mut group = CollisionGroups::new();
    group.set_membership(&[2]);
    group.set_blacklist(&[2]);

    let shape = ShapeHandle3::new(Cuboid::new(::na::Vector3::new(0.5f32, 0.5, 0.5)));
    let pos = ::na::Isometry3::new(::na::Vector3::new(pos[0], pos[1], 0.0), ::na::zero());
    let world_trans = {
        let trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 0.5f32)
            .to_superset();
        ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
    };

    let entity = world.create_entity().build();

    ::component::ColBody::add(world, entity, pos.clone(), shape, group);
    ::component::StaticDraw::add(world, entity, 1, world_trans);
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

