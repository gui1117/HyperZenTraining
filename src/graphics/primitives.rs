use vulkano::device::Queue;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::command_buffer::{CommandBufferExecFuture, AutoCommandBuffer};
use vulkano::sync::NowFuture;
use std::sync::Arc;
use super::Vertex;

pub fn instance_primitives(
    queue: Arc<Queue>,
) -> (Vec<Arc<ImmutableBuffer<[Vertex]>>>, Vec<CommandBufferExecFuture<NowFuture, AutoCommandBuffer>>) {

    let (sphere, sphere_future) = ImmutableBuffer::from_iter(
        {
            let sphere = ::ncollide::procedural::sphere(1.0, 16, 16, false);
            let indices = match sphere.indices {
                ::ncollide::procedural::IndexBuffer::Unified(ref indices) => indices.clone(),
                _ => unreachable!(),
            };

            let mut vertices = vec!();
            for p in indices {
                vertices.push(Vertex { position: [
                    sphere.coords[p.x as usize][0],
                    sphere.coords[p.x as usize][1],
                    sphere.coords[p.x as usize][2],
                ]});
                vertices.push(Vertex { position: [
                    sphere.coords[p.y as usize][0],
                    sphere.coords[p.y as usize][1],
                    sphere.coords[p.y as usize][2],
                ]});
                vertices.push(Vertex { position: [
                    sphere.coords[p.z as usize][0],
                    sphere.coords[p.z as usize][1],
                    sphere.coords[p.z as usize][2],
                ]});
            }
            vertices.into_iter()
        },
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (plane, plane_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0] },
            Vertex { position: [1.0, 1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (square_pyramid, square_pyramid_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },

            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },

            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },

            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },

            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },

            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (square_pyramid_base, square_pyramid_base_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },

            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (square_pyramid_side_1, square_pyramid_side_1_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (square_pyramid_side_2, square_pyramid_side_2_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (square_pyramid_side_3, square_pyramid_side_3_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (square_pyramid_side_4, square_pyramid_side_4_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (triangle_pyramid_base, triangle_pyramid_base_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -0.86602540378443864676, -1.0] },
            Vertex { position: [0.0, 0.86602540378443864676, -1.0] },
            Vertex { position: [1.0, -0.86602540378443864676, -1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (triangle_pyramid_side_1, triangle_pyramid_side_1_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -0.86602540378443864676, -1.0] },
            Vertex { position: [0.0, 0.86602540378443864676, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (triangle_pyramid_side_2, triangle_pyramid_side_2_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [0.0, 0.86602540378443864676, -1.0] },
            Vertex { position: [1.0, -0.86602540378443864676, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (triangle_pyramid_side_3, triangle_pyramid_side_3_future) = ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -0.86602540378443864676, -1.0] },
            Vertex { position: [1.0, -0.86602540378443864676, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    (
        vec![
            sphere,
            plane,

            square_pyramid,
            square_pyramid_base,
            square_pyramid_side_1,
            square_pyramid_side_2,
            square_pyramid_side_3,
            square_pyramid_side_4,

            triangle_pyramid_base,
            triangle_pyramid_side_1,
            triangle_pyramid_side_2,
            triangle_pyramid_side_3,
        ],
        vec![
            sphere_future,
            plane_future,

            square_pyramid_future,
            square_pyramid_base_future,
            square_pyramid_side_1_future,
            square_pyramid_side_2_future,
            square_pyramid_side_3_future,
            square_pyramid_side_4_future,

            triangle_pyramid_base_future,
            triangle_pyramid_side_1_future,
            triangle_pyramid_side_2_future,
            triangle_pyramid_side_3_future,
        ],
    )
}

#[allow(unused)]
pub mod primitive {
    pub const SPHERE:                  usize =  0;
    pub const PLANE:                   usize =  1;

    pub const SQUARE_PYRAMID:          usize =  2;
    pub const SQUARE_PYRAMID_BASE:     usize =  3;
    pub const SQUARE_PYRAMID_SIDE_1:   usize =  4;
    pub const SQUARE_PYRAMID_SIDE_2:   usize =  5;
    pub const SQUARE_PYRAMID_SIDE_3:   usize =  6;
    pub const SQUARE_PYRAMID_SIDE_4:   usize =  7;

    pub const TRIANGLE_PYRAMID_BASE:   usize =  8;
    pub const TRIANGLE_PYRAMID_SIDE_1: usize =  9;
    pub const TRIANGLE_PYRAMID_SIDE_2: usize = 10;
    pub const TRIANGLE_PYRAMID_SIDE_3: usize = 11;
}
