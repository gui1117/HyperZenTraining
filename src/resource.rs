pub use graphics::Data as Graphics;
pub use imgui::ImGui;
pub use maze::Maze;

pub type PhysicWorld = ::nphysics::world::World<f32>;
pub type WinitEvents = Vec<::winit::Event>;

pub struct Config {
    pub fps: u32,
    pub dt: f32,
    pub mouse_sensibility: f32,
}

// TODO: read from config file
impl Default for Config {
    fn default() -> Self {
        let fps = 60;
        Config {
            fps,
            dt: 1.0 / fps as f32,
            mouse_sensibility: 1000.0,
        }
    }
}

pub struct Rendering {
    pub image_num: Option<usize>,
    pub command_buffer: Option<::vulkano::command_buffer::AutoCommandBuffer>,
    pub second_command_buffer: Option<::vulkano::command_buffer::AutoCommandBuffer>,
    pub size_points: Option<(u32, u32)>,
    pub size_pixels: Option<(u32, u32)>,
}

impl Rendering {
    pub fn new() -> Self {
        Rendering {
            image_num: None,
            command_buffer: None,
            second_command_buffer: None,
            size_points: None,
            size_pixels: None,
        }
    }
}

// type InterferencesWithRayAlias<'a> = ::ncollide::world::InterferencesWithRay<'a, ::na::Point3<f32>, ::na::Isometry<f32, ::na::U3, ::na::Unit<::na::Quaternion<f32>>>, ::nphysics::object::WorldObject<f32>>;
// type InterferencesWithAABBAlias<'a> = ::ncollide::world::InterferencesWithAABB<'a, ::na::Point3<f32>, ::na::Isometry<f32, ::na::U3, ::na::Unit<::na::Quaternion<f32>>>, ::nphysics::object::WorldObject<f32>>;
// type InterferencesWithPointAlias<'a> = ::ncollide::world::InterferencesWithPoint<'a, ::na::Point3<f32>, ::na::Isometry<f32, ::na::U3, ::na::Unit<::na::Quaternion<f32>>>, ::nphysics::object::WorldObject<f32>>;
// type CollisionObjectAlias = ::ncollide::world::CollisionObject<::na::Point<f32, ::na::U3>, ::na::Isometry<f32, ::na::U3, ::na::Unit<::na::Quaternion<f32>>>, ::nphysics::object::WorldObject<f32>>;
// type RayIntersectionAlias = ::ncollide::query::RayIntersection<::na::Matrix<f32, ::na::U3, ::na::U1, ::na::MatrixArray<f32, ::na::U3, ::na::U1>>>;
// type FilterMapWithInterFnAlias<I> = fn((&CollisionObjectAlias, I)) -> Option<(::specs::Entity, ::std::cell::Ref<::nphysics::object::RigidBody<f32>>, I)>;
// type FilterMapFnAlias = fn(&CollisionObjectAlias) -> Option<(::specs::Entity, ::std::cell::Ref<::nphysics::object::RigidBody<f32>>)>;
// type FilterMapWithInterFnMutAlias<I> = fn((&CollisionObjectAlias, I)) -> Option<(::specs::Entity, ::std::cell::RefMut<::nphysics::object::RigidBody<f32>>, I)>;
// type FilterMapFnMutAlias = fn(&CollisionObjectAlias) -> Option<(::specs::Entity, ::std::cell::RefMut<::nphysics::object::RigidBody<f32>>)>;

// pub struct PhysicWorld(::nphysics::world::World<f32>);
// unsafe impl Send for PhysicWorld {}
// unsafe impl Sync for PhysicWorld {}

// // TODO: Intersection with sensor
// // TODO: impl mut intersection
// impl PhysicWorld {
//     pub fn new() -> Self {
//         PhysicWorld(::nphysics::world::World::new())
//     }

//     pub fn add_rigid_body(&mut self, body: ::nphysics::object::RigidBody<f32>) -> ::nphysics::object::RigidBodyHandle<f32> {
//         self.0.add_rigid_body(body)
//     }

//     pub fn step(&mut self, dt: f32) {
//         self.0.step(dt)
//     }

//     pub fn filter_map(physic_object: &CollisionObjectAlias) -> Option<(::specs::Entity, ::std::cell::Ref<::nphysics::object::RigidBody<f32>>)> {
//         if let ::nphysics::object::WorldObject::RigidBody(ref rigid_body_handle) = physic_object.data {
//             let rigid_body = rigid_body_handle.borrow();
//             let entity: ::specs::Entity = rigid_body.user_data().unwrap().downcast_ref::<::specs::Entity>().unwrap().clone();
//             Some((entity, rigid_body))
//         } else {
//             None
//         }
//     }

//     pub fn filter_map_with_inter<A>((physic_object, inter): (&CollisionObjectAlias, A)) -> Option<(::specs::Entity, ::std::cell::Ref<::nphysics::object::RigidBody<f32>>, A)> {
//         if let ::nphysics::object::WorldObject::RigidBody(ref rigid_body_handle) = physic_object.data {
//             let rigid_body = rigid_body_handle.borrow();
//             let entity: ::specs::Entity = rigid_body.user_data().unwrap().downcast_ref::<::specs::Entity>().unwrap().clone();
//             Some((entity, rigid_body, inter))
//         } else {
//             None
//         }
//     }

//     #[allow(dead_code)]
//     pub fn interferences_with_ray<'a>(&'a self, ray: &'a ::ncollide::query::Ray<::na::Point3<f32>>, group: &'a ::ncollide::world::CollisionGroups) -> ::std::iter::FilterMap<InterferencesWithRayAlias, FilterMapWithInterFnAlias<RayIntersectionAlias>> {
//         self.0.collision_world().interferences_with_ray(ray, group).filter_map(PhysicWorld::filter_map_with_inter::<RayIntersectionAlias>)
//     }

//     #[allow(dead_code)]
//     pub fn interferences_with_aabb<'a>(&'a self, aabb: &'a ::ncollide::bounding_volume::AABB<::na::Point3<f32>>, group: &'a ::ncollide::world::CollisionGroups) -> ::std::iter::FilterMap<InterferencesWithAABBAlias, FilterMapFnAlias> {
//         self.0.collision_world().interferences_with_aabb(aabb, group).filter_map(PhysicWorld::filter_map)
//     }

//     #[allow(dead_code)]
//     pub fn interferences_with_point<'a>(&'a self, point: &'a ::na::Point3<f32>, group: &'a ::ncollide::world::CollisionGroups) -> ::std::iter::FilterMap<InterferencesWithPointAlias, FilterMapFnAlias> {
//         self.0.collision_world().interferences_with_point(point, group).filter_map(PhysicWorld::filter_map)
//     }

//     pub fn filter_map_mut(physic_object: &CollisionObjectAlias) -> Option<(::specs::Entity, ::std::cell::RefMut<::nphysics::object::RigidBody<f32>>)> {
//         if let ::nphysics::object::WorldObject::RigidBody(ref rigid_body_handle) = physic_object.data {
//             let rigid_body = rigid_body_handle.borrow_mut();
//             let entity: ::specs::Entity = rigid_body.user_data().unwrap().downcast_ref::<::specs::Entity>().unwrap().clone();
//             Some((entity, rigid_body))
//         } else {
//             None
//         }
//     }

//     pub fn filter_map_with_inter_mut<A>((physic_object, inter): (&CollisionObjectAlias, A)) -> Option<(::specs::Entity, ::std::cell::RefMut<::nphysics::object::RigidBody<f32>>, A)> {
//         if let ::nphysics::object::WorldObject::RigidBody(ref rigid_body_handle) = physic_object.data {
//             let rigid_body = rigid_body_handle.borrow_mut();
//             let entity: ::specs::Entity = rigid_body.user_data().unwrap().downcast_ref::<::specs::Entity>().unwrap().clone();
//             Some((entity, rigid_body, inter))
//         } else {
//             None
//         }
//     }

//     #[allow(dead_code)]
//     pub fn mut_interferences_with_ray<'a>(&'a mut self, ray: &'a ::ncollide::query::Ray<::na::Point3<f32>>, group: &'a ::ncollide::world::CollisionGroups) -> ::std::iter::FilterMap<InterferencesWithRayAlias, FilterMapWithInterFnMutAlias<RayIntersectionAlias>> {
//         self.0.collision_world().interferences_with_ray(ray, group).filter_map(PhysicWorld::filter_map_with_inter_mut::<RayIntersectionAlias>)
//     }

//     #[allow(dead_code)]
//     pub fn mut_interferences_with_aabb<'a>(&'a mut self, aabb: &'a ::ncollide::bounding_volume::AABB<::na::Point3<f32>>, group: &'a ::ncollide::world::CollisionGroups) -> ::std::iter::FilterMap<InterferencesWithAABBAlias, FilterMapFnMutAlias> {
//         self.0.collision_world().interferences_with_aabb(aabb, group).filter_map(PhysicWorld::filter_map_mut)
//     }

//     #[allow(dead_code)]
//     pub fn mut_interferences_with_point<'a>(&'a mut self, point: &'a ::na::Point3<f32>, group: &'a ::ncollide::world::CollisionGroups) -> ::std::iter::FilterMap<InterferencesWithPointAlias, FilterMapFnMutAlias> {
//         self.0.collision_world().interferences_with_point(point, group).filter_map(PhysicWorld::filter_map_mut)
//     }
// }
