use std::sync::Arc;
use super::Vertex;

pub fn instance_primitives(device: Arc<::vulkano::device::Device>) -> Vec<Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<[Vertex]>>> {
    let plane = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        [
            Vertex { position: [-1.0, -1.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0] },
            Vertex { position: [1.0, 1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let pyramid = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
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
    ).expect("failed to create buffer");

    let pyramid_base = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },

            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let pyramid_side_1 = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        [
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let pyramid_side_2 = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        [
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let pyramid_side_3 = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        [
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let pyramid_side_4 = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(),
        ::vulkano::buffer::BufferUsage::vertex_buffer(),
        [
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [0.0, 0.0, 1.0] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    vec!(
        plane,
        pyramid,
        pyramid_base,
        pyramid_side_1,
        pyramid_side_2,
        pyramid_side_3,
        pyramid_side_4,
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
