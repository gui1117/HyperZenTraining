use std::sync::Arc;
use super::Vertex;

pub fn instance_primitives(queue: Arc<::vulkano::device::Queue>) -> (Vec<Arc<::vulkano::buffer::immutable::ImmutableBuffer<[Vertex]>>>, Vec<::vulkano::command_buffer::CommandBufferExecFuture<::vulkano::sync::NowFuture, ::vulkano::command_buffer::AutoCommandBuffer>>) {
    let (plane, plane_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0] },
            Vertex { position: [1.0, 1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0] },
        ].iter()
            .cloned(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (pyramid, pyramid_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
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
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (pyramid_base, pyramid_base_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },

            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
        ].iter()
            .cloned(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (pyramid_side_1, pyramid_side_1_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (pyramid_side_2, pyramid_side_2_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
        [
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (pyramid_side_3, pyramid_side_3_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
        [
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let (pyramid_side_4, pyramid_side_4_future) = ::vulkano::buffer::immutable::ImmutableBuffer::from_iter(
        [
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    (
        vec!(
            plane,
            pyramid,
            pyramid_base,
            pyramid_side_1,
            pyramid_side_2,
            pyramid_side_3,
            pyramid_side_4,
        ),
        vec!(
            plane_future,
            pyramid_future,
            pyramid_base_future,
            pyramid_side_1_future,
            pyramid_side_2_future,
            pyramid_side_3_future,
            pyramid_side_4_future,
        )
    )
}

#[allow(unused)]
pub mod primitive {
    pub const PLANE: usize = 0;
    pub const PYRAMID: usize = 1;
    pub const PYRAMID_BASE: usize = 2;
    pub const PYRAMID_SIDE_1: usize = 3;
    pub const PYRAMID_SIDE_2: usize = 4;
    pub const PYRAMID_SIDE_3: usize = 5;
    pub const PYRAMID_SIDE_4: usize = 6;
}
