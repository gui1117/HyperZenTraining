pub type Group = ::ncollide::world::CollisionGroups;
pub type Point = ::na::Point<f32, ::na::U3>;
pub type Position = ::na::Isometry<f32, ::na::U3, ::na::Unit<::na::Quaternion<f32>>>;
pub type Shape = ::ncollide::shape::ShapeHandle<Point, Position>;
pub type World = ::ncollide::world::CollisionWorld<::na::Point<f32, ::na::U3>, Position, Data>;
// pub type Object = ::ncollide::world::CollisionObject<Point, Position, Data>;
pub type Data = ::specs::Entity;
