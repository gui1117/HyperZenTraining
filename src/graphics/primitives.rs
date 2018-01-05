use vulkano::device::Queue;
use vulkano::buffer::{BufferUsage, ImmutableBuffer};
use vulkano::sync::{now, GpuFuture};
use std::sync::Arc;
use super::Vertex;
use super::DebugVertex;
use wavefront_obj::obj;

use std::f32::consts::PI;

const HOOK_LINKS: usize = 50;

pub fn instance_primitives(
    queue: Arc<Queue>,
) -> (Vec<Vec<Arc<ImmutableBuffer<[Vertex]>>>>, Box<GpuFuture>) {
    let mut primitives_buffers_def = vec![];

    // Plane
    primitives_buffers_def.push(vec![
        vec![
            [-1.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [1.0, -1.0, 0.0],
        ],
    ]);

    // Square pyramid
    primitives_buffers_def.push(vec![
        vec![
            [-1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0],
            //
            [1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
        vec![[-1.0, 1.0, -1.0], [-1.0, -1.0, -1.0], [0.0, 0.0, 1.0]],
        vec![[1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [0.0, 0.0, 1.0]],
        vec![[1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [0.0, 0.0, 1.0]],
        vec![[-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [0.0, 0.0, 1.0]],
    ]);

    // Triangle pyramid
    primitives_buffers_def.push(vec![
        vec![
            [-1.0, -0.86602540378443864676, -1.0],
            [0.0, 0.86602540378443864676, -1.0],
            [1.0, -0.86602540378443864676, -1.0],
        ],
        vec![
            [-1.0, -0.86602540378443864676, -1.0],
            [0.0, 0.86602540378443864676, -1.0],
            [0.0, 0.0, 1.0],
        ],
        vec![
            [0.0, 0.86602540378443864676, -1.0],
            [1.0, -0.86602540378443864676, -1.0],
            [0.0, 0.0, 1.0],
        ],
        vec![
            [-1.0, -0.86602540378443864676, -1.0],
            [1.0, -0.86602540378443864676, -1.0],
            [0.0, 0.0, 1.0],
        ],
    ]);

    // Sphere
    let sphere_vertices = {
        let sphere = ::ncollide::procedural::sphere(1.0, 16, 16, false);
        let indices = match sphere.indices {
            ::ncollide::procedural::IndexBuffer::Unified(ref indices) => indices.clone(),
            _ => unreachable!(),
        };

        let mut vertices = vec![];
        for p in indices {
            vertices.push([
                sphere.coords[p.x as usize][0] * 2.0,
                sphere.coords[p.x as usize][1] * 2.0,
                sphere.coords[p.x as usize][2] * 2.0,
            ]);
            vertices.push([
                sphere.coords[p.y as usize][0] * 2.0,
                sphere.coords[p.y as usize][1] * 2.0,
                sphere.coords[p.y as usize][2] * 2.0,
            ]);
            vertices.push([
                sphere.coords[p.z as usize][0] * 2.0,
                sphere.coords[p.z as usize][1] * 2.0,
                sphere.coords[p.z as usize][2] * 2.0,
            ]);
        }

        vertices
    };

    primitives_buffers_def.push(vec![sphere_vertices]);

    // Six
    let mut six_buffers_def = vec![vec![], vec![]];
    for i in 0..6 {
        let a0 = (i as f32 - 0.5) * 2.0 * PI / 6.0;
        let a1 = ((i + 1) as f32 - 0.5) * 2.0 * PI / 6.0;

        let p0 = [a0.cos(), a0.sin()];
        let p1 = [a1.cos(), a1.sin()];

        six_buffers_def[0].push([p1[0], p1[1], -1.0]);
        six_buffers_def[0].push([p0[0], p0[1], -1.0]);
        six_buffers_def[0].push([0.0, 0.0, -1.0]);

        six_buffers_def[1].push([p0[0], p0[1], 1.0]);
        six_buffers_def[1].push([p1[0], p1[1], 1.0]);
        six_buffers_def[1].push([0.0, 0.0, 1.0]);

        six_buffers_def.push(vec![
            [p0[0], p0[1], 1.0],
            [p0[0], p0[1], -1.0],
            [p1[0], p1[1], 1.0],
            [p0[0], p0[1], -1.0],
            [p1[0], p1[1], -1.0],
            [p1[0], p1[1], 1.0],
        ]);
    }
    primitives_buffers_def.push(six_buffers_def);

    // Cube
    primitives_buffers_def.push(vec![
        // Floor
        vec![
            [1.0, -1.0, -1.0],
            [-1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            //
            [1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
        // Ceil
        vec![
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [-1.0, 1.0, 1.0],
            //
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ],
        // Left
        vec![
            [-1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, -1.0],
            //
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0],
        ],
        // Right
        vec![
            [1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, -1.0],
            //
            [1.0, 1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, -1.0],
        ],
        // Back
        vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            //
            [1.0, -1.0, 1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
        ],
        // Front
        vec![
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, 1.0, 1.0],
            //
            [-1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0],
        ],
    ]);

    // Cylinder
    let cylinder_div = 32;
    let mut cylinder_buffers_def = vec![vec![]];
    for i in 0..cylinder_div {
        let a0 = (i as f32) * 2.0 * PI / cylinder_div as f32;
        let a1 = ((i + 1) as f32) * 2.0 * PI / cylinder_div as f32;

        let p0 = [a0.cos(), a0.sin()];
        let p1 = [a1.cos(), a1.sin()];

        cylinder_buffers_def[0].push([p0[0], p0[1], -1.0]);
        cylinder_buffers_def[0].push([p1[0], p1[1], -1.0]);
        cylinder_buffers_def[0].push([0.0, 0.0, -1.0]);

        cylinder_buffers_def[0].push([p0[0], p0[1], 1.0]);
        cylinder_buffers_def[0].push([p1[0], p1[1], 1.0]);
        cylinder_buffers_def[0].push([0.0, 0.0, 1.0]);

        cylinder_buffers_def[0].push([p0[0], p0[1], -1.0]);
        cylinder_buffers_def[0].push([p0[0], p0[1], 1.0]);
        cylinder_buffers_def[0].push([p1[0], p1[1], 1.0]);

        cylinder_buffers_def[0].push([p0[0], p0[1], -1.0]);
        cylinder_buffers_def[0].push([p1[0], p1[1], -1.0]);
        cylinder_buffers_def[0].push([p1[0], p1[1], 1.0]);
    }
    primitives_buffers_def.push(cylinder_buffers_def);

    // Cube pitted
    let pit_radius = 0.4;
    primitives_buffers_def.push(vec![
        // Floor
        vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            //
            [1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
        // Inner floor
        vec![
            [-pit_radius, -pit_radius, 1.0 - pit_radius],
            [pit_radius, -pit_radius, 1.0 - pit_radius],
            [-pit_radius, pit_radius, 1.0 - pit_radius],
            //
            [pit_radius, pit_radius, 1.0 - pit_radius],
            [pit_radius, -pit_radius, 1.0 - pit_radius],
            [-pit_radius, pit_radius, 1.0 - pit_radius],
        ],
        // Left
        vec![
            [-1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, -1.0],
            //
            [-1.0, 1.0, 1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, -1.0],
        ],
        // Inner left
        vec![
            [-pit_radius, -pit_radius, 1.0 - pit_radius],
            [-pit_radius, -pit_radius, 1.0 + pit_radius],
            [-pit_radius, pit_radius, 1.0 - pit_radius],
            //
            [-pit_radius, pit_radius, 1.0 + pit_radius],
            [-pit_radius, -pit_radius, 1.0 + pit_radius],
            [-pit_radius, pit_radius, 1.0 - pit_radius],
        ],
        // Right
        vec![
            [1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, -1.0],
            //
            [1.0, 1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, -1.0],
        ],
        // Inner right
        vec![
            [pit_radius, -pit_radius, 1.0 - pit_radius],
            [pit_radius, -pit_radius, 1.0 + pit_radius],
            [pit_radius, pit_radius, 1.0 - pit_radius],
            //
            [pit_radius, pit_radius, 1.0 + pit_radius],
            [pit_radius, -pit_radius, 1.0 + pit_radius],
            [pit_radius, pit_radius, 1.0 - pit_radius],
        ],
        // Back
        vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            //
            [1.0, -1.0, 1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
        ],
        // Inner back
        vec![
            [-pit_radius, -pit_radius, 1.0 - pit_radius],
            [pit_radius, -pit_radius, 1.0 - pit_radius],
            [-pit_radius, -pit_radius, 1.0 + pit_radius],
            //
            [pit_radius, -pit_radius, 1.0 + pit_radius],
            [-pit_radius, -pit_radius, 1.0 + pit_radius],
            [pit_radius, -pit_radius, 1.0 - pit_radius],
        ],
        // Front
        vec![
            [-1.0, 1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, 1.0],
            //
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0],
        ],
        // Inner front
        vec![
            [-pit_radius, pit_radius, 1.0 - pit_radius],
            [pit_radius, pit_radius, 1.0 - pit_radius],
            [-pit_radius, pit_radius, 1.0 + pit_radius],
            //
            [pit_radius, pit_radius, 1.0 + pit_radius],
            [-pit_radius, pit_radius, 1.0 + pit_radius],
            [pit_radius, pit_radius, 1.0 - pit_radius],
        ],
        // Ceil
        vec![
            // Minor rectangle
            [-1.0, -1.0, 1.0],
            [-pit_radius, -1.0, 1.0],
            [-1.0, 1.0, 1.0],
            //
            [-pit_radius, 1.0, 1.0],
            [-pit_radius, -1.0, 1.0],
            [-1.0, 1.0, 1.0],
            // Major rectangle
            [1.0, 1.0, 1.0],
            [pit_radius, 1.0, 1.0],
            [1.0, -1.0, 1.0],
            //
            [pit_radius, -1.0, 1.0],
            [pit_radius, 1.0, 1.0],
            [1.0, -1.0, 1.0],
            // Minor square
            [-pit_radius, -1.0, 1.0],
            [pit_radius, -1.0, 1.0],
            [-pit_radius, -pit_radius, 1.0],
            //
            [pit_radius, -pit_radius, 1.0],
            [pit_radius, -1.0, 1.0],
            [-pit_radius, -pit_radius, 1.0],
            // Minor square
            [pit_radius, 1.0, 1.0],
            [-pit_radius, 1.0, 1.0],
            [pit_radius, pit_radius, 1.0],
            //
            [-pit_radius, pit_radius, 1.0],
            [-pit_radius, 1.0, 1.0],
            [pit_radius, pit_radius, 1.0],
        ],
    ]);

    // Link oriented along axis y
    let radius = 0.02;
    let width = 0.04;
    let link = vec![
        // Floor
        vec![
            [width + radius, -1.0, -radius],
            [-width - radius, -1.0, -radius],
            [width + radius, -1.0 + radius * 2.0, -radius],
            //
            [-width - radius, -1.0, -radius],
            [-width - radius, -1.0 + radius * 2.0, -radius],
            [width + radius, -1.0 + radius * 2.0, -radius],
            //
            [-width - radius, 1.0, -radius],
            [width + radius, 1.0, -radius],
            [width + radius, 1.0 - radius * 2.0, -radius],
            //
            [-width - radius, 1.0 - radius * 2.0, -radius],
            [-width - radius, 1.0, -radius],
            [width + radius, 1.0 - radius * 2.0, -radius],
            //
            [-width + radius, -1.0 + radius * 2.0, -radius],
            [-width - radius, -1.0 + radius * 2.0, -radius],
            [-width + radius, 1.0 - radius * 2.0, -radius],
            //
            [-width + radius, 1.0 - radius * 2.0, -radius],
            [-width - radius, -1.0 + radius * 2.0, -radius],
            [-width - radius, 1.0 - radius * 2.0, -radius],
            //
            [width + radius, -1.0 + radius * 2.0, -radius],
            [width - radius, -1.0 + radius * 2.0, -radius],
            [width + radius, 1.0 - radius * 2.0, -radius],
            //
            [width + radius, 1.0 - radius * 2.0, -radius],
            [width - radius, -1.0 + radius * 2.0, -radius],
            [width - radius, 1.0 - radius * 2.0, -radius],
        ],
        // Ceil
        vec![
            [-width - radius, -1.0, radius],
            [width + radius, -1.0, radius],
            [width + radius, -1.0 + radius * 2.0, radius],
            //
            [-width - radius, -1.0 + radius * 2.0, radius],
            [-width - radius, -1.0, radius],
            [width + radius, -1.0 + radius * 2.0, radius],
            //
            [width + radius, 1.0, radius],
            [-width - radius, 1.0, radius],
            [width + radius, 1.0 - radius * 2.0, radius],
            //
            [-width - radius, 1.0, radius],
            [-width - radius, 1.0 - radius * 2.0, radius],
            [width + radius, 1.0 - radius * 2.0, radius],
            //
            [-width - radius, -1.0 + radius * 2.0, radius],
            [-width + radius, -1.0 + radius * 2.0, radius],
            [-width + radius, 1.0 - radius * 2.0, radius],
            //
            [-width - radius, -1.0 + radius * 2.0, radius],
            [-width + radius, 1.0 - radius * 2.0, radius],
            [-width - radius, 1.0 - radius * 2.0, radius],
            //
            [width - radius, -1.0 + radius * 2.0, radius],
            [width + radius, -1.0 + radius * 2.0, radius],
            [width + radius, 1.0 - radius * 2.0, radius],
            //
            [width - radius, -1.0 + radius * 2.0, radius],
            [width + radius, 1.0 - radius * 2.0, radius],
            [width - radius, 1.0 - radius * 2.0, radius],
        ],
        vec![
            [-width - radius, -1.0, -radius],
            [width + radius, -1.0, -radius],
            [width + radius, -1.0, radius],
            //
            [-width - radius, -1.0, radius],
            [-width - radius, -1.0, -radius],
            [width + radius, -1.0, radius],
        ],
        vec![
            [width + radius, 1.0, -radius],
            [-width - radius, 1.0, -radius],
            [width + radius, 1.0, radius],
            //
            [-width - radius, 1.0, -radius],
            [-width - radius, 1.0, radius],
            [width + radius, 1.0, radius],
        ],
        vec![
            [-width - radius, 1.0, -radius],
            [-width - radius, -1.0, -radius],
            [-width - radius, -1.0, radius],
            //
            [-width - radius, 1.0, radius],
            [-width - radius, 1.0, -radius],
            [-width - radius, -1.0, radius],
        ],
        vec![
            [width + radius, -1.0, -radius],
            [width + radius, 1.0, -radius],
            [width + radius, -1.0, radius],
            //
            [width + radius, 1.0, -radius],
            [width + radius, 1.0, radius],
            [width + radius, -1.0, radius],
        ],
        vec![
            [width - radius, -1.0 + radius * 2.0, -radius],
            [-width + radius, -1.0 + radius * 2.0, -radius],
            [width - radius, -1.0 + radius * 2.0, radius],
            //
            [-width + radius, -1.0 + radius * 2.0, -radius],
            [-width + radius, -1.0 + radius * 2.0, radius],
            [width - radius, -1.0 + radius * 2.0, radius],
        ],
        vec![
            [-width + radius, 1.0 - radius * 2.0, -radius],
            [width - radius, 1.0 - radius * 2.0, -radius],
            [width - radius, 1.0 - radius * 2.0, radius],
            //
            [-width + radius, 1.0 - radius * 2.0, radius],
            [-width + radius, 1.0 - radius * 2.0, -radius],
            [width - radius, 1.0 - radius * 2.0, radius],
        ],
        vec![
            [-width + radius, -1.0 + radius * 2.0, -radius],
            [-width + radius, 1.0 - radius * 2.0, -radius],
            [-width + radius, -1.0 + radius * 2.0, radius],
            //
            [-width + radius, 1.0 - radius * 2.0, -radius],
            [-width + radius, 1.0 - radius * 2.0, radius],
            [-width + radius, -1.0 + radius * 2.0, radius],
        ],
        vec![
            [width - radius, 1.0 - radius * 2.0, -radius],
            [width - radius, -1.0 + radius * 2.0, -radius],
            [width - radius, -1.0 + radius * 2.0, radius],
            //
            [width - radius, 1.0 - radius * 2.0, radius],
            [width - radius, 1.0 - radius * 2.0, -radius],
            [width - radius, -1.0 + radius * 2.0, radius],
        ],
    ];

    let mut hook = vec![];
    for i in 0..HOOK_LINKS {
        let mut link = link.clone();
        if i % 2 == 1 {
            link.iter_mut().for_each(|v| {
                v.iter_mut().for_each(|p| {
                    let tmp = p[0];
                    p[0] = p[2];
                    p[2] = tmp;
                });
                v.reverse();
            });
        }
        link.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|p| {
                p[1] += (2.0 - 4.0*radius)*i as f32 + 1.0;
            });
        });
        hook.append(&mut link);
    }
    primitives_buffers_def.push(hook);

    let mut final_future = Box::new(now(queue.device().clone())) as Box<GpuFuture>;
    let mut primitives_buffers = vec![];
    for primitive_buffers_def in primitives_buffers_def {
        let mut primitive_buffers = vec![];
        for buffer_def in primitive_buffers_def {
            let (buffer, future) = ImmutableBuffer::from_iter(
                buffer_def
                    .iter()
                    .cloned()
                    .map(|position| Vertex { position }),
                BufferUsage::vertex_buffer(),
                queue.clone(),
            ).expect("failed to create buffer");

            primitive_buffers.push(buffer);
            final_future = Box::new(final_future.join(future)) as Box<GpuFuture>;
        }
        primitives_buffers.push(primitive_buffers);
    }

    (primitives_buffers, final_future)
}

#[allow(unused)]
pub mod primitive {
    // TODO: We don't need an atomic usize
    pub enum Primitive {
        Plane,
        SquarePyramid,
        TrianglePyramid,
        Sphere,
        Six,
        Cube,
        Cylinder,
        PitCube,
        Hook,
    }

    impl Primitive {
        pub fn index(&self) -> usize {
            match *self {
                Primitive::Plane => 0,
                Primitive::SquarePyramid => 1,
                Primitive::TrianglePyramid => 2,
                Primitive::Sphere => 3,
                Primitive::Six => 4,
                Primitive::Cube => 5,
                Primitive::Cylinder => 6,
                Primitive::PitCube => 7,
                Primitive::Hook => 8,
            }
        }

        fn groups_size(&self) -> usize {
            match *self {
                Primitive::Plane => 1,
                Primitive::SquarePyramid => 5,
                Primitive::TrianglePyramid => 4,
                Primitive::Sphere => 1,
                Primitive::Six => 8,
                Primitive::Cube => 6,
                Primitive::Cylinder => 1,
                Primitive::PitCube => 11,
                Primitive::Hook => 10*super::HOOK_LINKS,
            }
        }

        pub fn reserve(&self, size: usize) -> Vec<Vec<u16>> {
            let groups_size = self.groups_size();
            (0..size)
                .map(|_| GROUP_COUNTER.instantiate(groups_size))
                .collect()
        }

        pub fn instantiate(&self) -> (usize, Vec<u16>) {
            (self.index(), GROUP_COUNTER.instantiate(self.groups_size()))
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
            GroupCounter {
                counter: ::std::sync::atomic::AtomicUsize::new(1),
            }
        }

        fn next(&self) -> u16 {
            self.counter
                .fetch_add(1, ::std::sync::atomic::Ordering::Relaxed) as u16
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
