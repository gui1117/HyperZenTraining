use vulkano::device::Queue;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::sync::GpuFuture;
use std::sync::Arc;
use super::Vertex;
use super::DebugVertex;
use wavefront_obj::obj;

use std::f32::consts::PI;

pub fn instance_primitives(
    queue: Arc<Queue>,
) -> (Vec<Vec<Arc<ImmutableBuffer<[Vertex]>>>>, Box<GpuFuture>) {
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

    let (square_pyramid_side_1, square_pyramid_side_1_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [-1.0, -1.0, -1.0] },
                Vertex { position: [-1.0, 1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (square_pyramid_side_2, square_pyramid_side_2_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [-1.0, 1.0, -1.0] },
                Vertex { position: [1.0, 1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (square_pyramid_side_3, square_pyramid_side_3_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [1.0, 1.0, -1.0] },
                Vertex { position: [1.0, -1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (square_pyramid_side_4, square_pyramid_side_4_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [1.0, -1.0, -1.0] },
                Vertex { position: [-1.0, -1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (triangle_pyramid_base, triangle_pyramid_base_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [-1.0, -0.86602540378443864676, -1.0] },
                Vertex { position: [0.0, 0.86602540378443864676, -1.0] },
                Vertex { position: [1.0, -0.86602540378443864676, -1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (triangle_pyramid_side_1, triangle_pyramid_side_1_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [-1.0, -0.86602540378443864676, -1.0] },
                Vertex { position: [0.0, 0.86602540378443864676, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (triangle_pyramid_side_2, triangle_pyramid_side_2_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [0.0, 0.86602540378443864676, -1.0] },
                Vertex { position: [1.0, -0.86602540378443864676, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let (triangle_pyramid_side_3, triangle_pyramid_side_3_future) =
        ImmutableBuffer::from_iter(
            [
                Vertex { position: [-1.0, -0.86602540378443864676, -1.0] },
                Vertex { position: [1.0, -0.86602540378443864676, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
            BufferUsage::vertex_buffer(),
            queue.clone(),
        ).expect("failed to create buffer");

    let sphere_vertices = {
        let sphere = ::ncollide::procedural::sphere(1.0, 16, 16, false);
        let indices = match sphere.indices {
            ::ncollide::procedural::IndexBuffer::Unified(ref indices) => indices.clone(),
            _ => unreachable!(),
        };

        let mut vertices = vec![];
        for p in indices {
            vertices.push(Vertex {
                position: [
                    sphere.coords[p.x as usize][0] * 2.0,
                    sphere.coords[p.x as usize][1] * 2.0,
                    sphere.coords[p.x as usize][2] * 2.0,
                ],
            });
            vertices.push(Vertex {
                position: [
                    sphere.coords[p.y as usize][0] * 2.0,
                    sphere.coords[p.y as usize][1] * 2.0,
                    sphere.coords[p.y as usize][2] * 2.0,
                ],
            });
            vertices.push(Vertex {
                position: [
                    sphere.coords[p.z as usize][0] * 2.0,
                    sphere.coords[p.z as usize][1] * 2.0,
                    sphere.coords[p.z as usize][2] * 2.0,
                ],
            });
        }

        vertices
    };

    let (sphere, sphere_future) = ImmutableBuffer::from_iter(
        sphere_vertices.iter().cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    let nine_vertices = {
        let mut vertices = vec![];
        for i in 0..9 {
            let a0 = i as f32 * 2.0*PI/9.0;
            let a1 = (i+1) as f32 * 2.0*PI/9.0;

            let p0 = [a0.cos(), a0.sin()];
            let p1 = [a1.cos(), a1.sin()];

            vertices.push(Vertex { position: [p0[0], p0[1], -1.0]});
            vertices.push(Vertex { position: [p1[0], p1[1], -1.0]});
            vertices.push(Vertex { position: [0.0, 0.0, -1.0]});

            vertices.push(Vertex { position: [p0[0], p0[1], -1.0]});
            vertices.push(Vertex { position: [p1[0], p1[1], -1.0]});
            vertices.push(Vertex { position: [p1[0], p1[1], 1.0]});

            vertices.push(Vertex { position: [p0[0], p0[1], -1.0]});
            vertices.push(Vertex { position: [p0[0], p0[1], 1.0]});
            vertices.push(Vertex { position: [p1[0], p1[1], 1.0]});

            vertices.push(Vertex { position: [p0[0], p0[1], 1.0]});
            vertices.push(Vertex { position: [p1[0], p1[1], 1.0]});
            vertices.push(Vertex { position: [0.0, 0.0, 1.0]});
        }
        vertices
    };

    let (nine, nine_future) = ImmutableBuffer::from_iter(
        nine_vertices.iter().cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");

    (
        vec![
            vec![plane],

            vec![
                square_pyramid_base,
                square_pyramid_side_1,
                square_pyramid_side_2,
                square_pyramid_side_3,
                square_pyramid_side_4,
            ],

            vec![
                triangle_pyramid_base,
                triangle_pyramid_side_1,
                triangle_pyramid_side_2,
                triangle_pyramid_side_3,
            ],

            vec![
                sphere,
            ],

            vec![
                nine,
            ],
        ],
        Box::new(plane_future
            .join(square_pyramid_base_future)
            .join(square_pyramid_side_1_future)
            .join(square_pyramid_side_2_future)
            .join(square_pyramid_side_3_future)
            .join(square_pyramid_side_4_future)
            .join(triangle_pyramid_base_future)
            .join(triangle_pyramid_side_1_future)
            .join(triangle_pyramid_side_2_future)
            .join(triangle_pyramid_side_3_future)
            .join(sphere_future)
            .join(nine_future)
        ) as Box<GpuFuture>
    )
}

#[allow(unused)]
pub mod primitive {
    pub enum Primitive {
        Plane,
        SquarePyramid,
        TrianglePyramid,
        Sphere,
        Nine,
    }

    impl Primitive {
        pub fn instantiate(&self) -> (usize, Vec<u16>) {
            match *self {
                Primitive::Plane => (0, GROUP_COUNTER.instantiate(1)),
                Primitive::SquarePyramid => (1, GROUP_COUNTER.instantiate(5)),
                Primitive::TrianglePyramid => (2, GROUP_COUNTER.instantiate(4)),
                Primitive::Sphere => (3, GROUP_COUNTER.instantiate(1)),
                Primitive::Nine => (4, GROUP_COUNTER.instantiate(1)),
            }
        }
    }

    lazy_static! {
        static ref GROUP_COUNTER: GroupCounter = GroupCounter::new();
    }

    pub const GROUP_COUNTER_SIZE: usize = 65536;

    struct GroupCounter {
        counter: ::std::sync::atomic::AtomicUsize,
    }

    impl GroupCounter {
        fn new() -> Self {
            GroupCounter { counter: ::std::sync::atomic::AtomicUsize::new(1) }
        }

        fn next(&self) -> u16 {
            self.counter.fetch_add(
                1,
                ::std::sync::atomic::Ordering::Relaxed,
            ) as u16
        }

        fn instantiate(&self, n: usize) -> Vec<u16> {
            (0..n).map(|_| self.next()).collect()
        }
    }
}

pub fn load_debug_arrow(
    queue: Arc<Queue>,
) -> (Arc<ImmutableBuffer<[DebugVertex]>>, Box<GpuFuture>) {
    let arrow = obj::parse(include_str!("arrow.obj").into()).unwrap();

    let mut vertices = vec![];
    for object in &arrow.objects {
        assert!(object.geometry.len() == 1);
        for shape in &object.geometry[0].shapes {
            let indexes = match shape.primitive {
                obj::Primitive::Triangle(a, b, c) => [
                    (a.0, a.2.unwrap()),
                    (b.0, b.2.unwrap()),
                    (c.0, c.2.unwrap()),
                ],
                _ => panic!("arrow obj not handled"),
            };
            for &(v, n) in &indexes {
                vertices.push(DebugVertex {
                    position: [
                        object.vertices[v].x as f32,
                        object.vertices[v].y as f32,
                        object.vertices[v].z as f32,
                    ],
                    normal: [
                        object.normals[n].x as f32,
                        object.normals[n].y as f32,
                        object.normals[n].z as f32,
                    ],
                });
            }
        }
    }

    let res = ImmutableBuffer::from_iter(
        vertices.iter().cloned(),
        BufferUsage::vertex_buffer(),
        queue.clone(),
    ).expect("failed to create buffer");
    (res.0, Box::new(res.1) as Box<GpuFuture>)
}
