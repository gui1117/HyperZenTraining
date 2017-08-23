extern crate winit;
extern crate vulkano_win;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate fps_clock;
extern crate alga;

extern crate nalgebra as na;
extern crate ncollide;

use vulkano_win::VkSurfaceBuild;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::image::ImageUsage;
use vulkano::instance::Instance;
use vulkano::instance::ApplicationInfo;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::CompositeAlpha;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use na::Vector3;
use ncollide::world::{CollisionWorld, CollisionGroups, GeometricQueryType, CollisionObject3};
use ncollide::narrow_phase::{ProximityHandler, ContactHandler, ContactAlgorithm3};
use ncollide::shape::{Plane, Ball, Cylinder, Cuboid, ShapeHandle3};
use ncollide::query::Proximity;
use alga::general::SubsetOf;
use alga::linear::AffineTransformation;

use std::iter;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 3],
}
impl_vertex!(Vertex, position);

mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450

layout(location = 0) in vec3 position;

layout(set = 0, binding = 0) uniform View {
    mat4 view;
    mat4 proj;
} view;

layout(set = 1, binding = 0) uniform World {
    mat4 world;
} world;


void main() {
    gl_Position = view.proj * view.view * world.world * vec4(position, 1.0);
}
"]
    struct Dummy;
}

mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
#version 450

// layout(location = 0) out vec4 f_color;

layout(push_constant) uniform Group {
    uint group;
} group;

void main() {
    // f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"]
    struct Dummy;
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl Direction {
    #[inline]
    fn orthogonal(self, other: Self) -> bool {
        use Direction::*;
        match (self, other) {
            (Forward, Forward) | (Forward, Backward) | (Backward, Forward) | (Backward, Backward) => false,
            _ => true,
        }
    }

    #[inline]
    fn perpendicular(self, other: Self) -> bool {
        !self.orthogonal(other)
    }
}

fn main() {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        let info = ApplicationInfo::from_cargo_toml();
        Instance::new(Some(&info), &extensions, None).expect("failed to create Vulkan instance")
    };

    //TODO: read config and save device
    let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    window.window().set_cursor(winit::MouseCursor::NoneCursor);
    window.window().set_cursor_state(winit::CursorState::Grab).unwrap();

    let queue = physical
        .queue_families()
        .find(|&q| {
            q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
        })
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue, 0.5)].iter().cloned(),
        ).expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    let (swapchain, images) = {
        let caps = window.surface().capabilities(physical).expect(
            "failed to get surface capabilities",
        );

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let image_usage = ImageUsage {
            sampled: true,
            color_attachment: true,
            ..ImageUsage::none()
        };

        Swapchain::new(
            device.clone(),
            window.surface().clone(),
            caps.min_image_count,
            Format::B8G8R8A8Srgb,
            dimensions,
            1,
            image_usage,
            &queue,
            SurfaceTransform::Identity,
            CompositeAlpha::Opaque,
            PresentMode::Fifo,
            true,
            None,
        ).expect("failed to create swapchain")
    };

    let depth_buffer = vulkano::image::attachment::AttachmentImage::transient(device.clone(), images[0].dimensions(), vulkano::format::D16Unorm).unwrap();

    let cuboid_vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::vertex_buffer(),
        Some(queue.family()),
        [
            Vertex { position: [-1.0f32, 1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let render_pass = Arc::new(
        ordered_passes_renderpass!(device.clone(),
        attachments: {
            color: {
                load: DontCare,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: vulkano::format::Format::D16Unorm,
                samples: 1,
            }
        },
        passes: [
            {
                color: [],
                depth_stencil: {depth},
                input: []
            },
            {
                color: [color],
                depth_stencil: {},
                input: []
            }
        ]
    ).unwrap(),
    );

    let width = images[0].dimensions()[0];
    let height = images[0].dimensions()[1];

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .viewports(iter::once(Viewport {
                origin: [0.0, 0.0],
                depth_range: 0.0..1.0,
                dimensions: [width as f32, height as f32],
            }))
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let final_pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .viewports(iter::once(Viewport {
                origin: [0.0, 0.0],
                depth_range: 0.0..1.0,
                dimensions: [width as f32, height as f32],
            }))
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 1).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let framebuffers = images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone()).unwrap()
                    .add(depth_buffer.clone()).unwrap()
                    .build().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;

    let mut fps = fps_clock::FpsClock::new(30);

    let mut world = CollisionWorld::new(0.02, false);

    let character = ShapeHandle3::new(Cylinder::new(0.5f32, 0.3));
    let character_pos = na::Isometry3::new(Vector3::new(1.0, 0.0, 0.0), na::ColumnVector::z());
    let character_groups = CollisionGroups::new();
    world.deferred_add(0, character_pos, character, character_groups, GeometricQueryType::Contacts(0.0), ());

    let proj_matrix = na::Perspective3::new(
        images[0].dimensions()[1] as f32 / images[0].dimensions()[0] as f32,
        ::std::f32::consts::FRAC_PI_3,
        0.01,
        100.0,
    ).unwrap();

    let view_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::View>::from_data(
            device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            Some(queue.family()),
            vs::ty::View {
                view: na::Matrix4::identity().into(), // This is computed at each frame
                proj: proj_matrix.into(),
            },
        ).expect("failed to create buffer");

    let view_set =
        Arc::new(
            vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(pipeline.clone(), 0)
                .add_buffer(view_uniform_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

    let mut wall_kind_groups = CollisionGroups::new();
    wall_kind_groups.set_membership(&[2]);
    wall_kind_groups.set_blacklist(&[2]);

    // let floor = ShapeHandle3::new(Plane::new(Vector3::new(0.0, 0.0, 1.0)));
    // world.deferred_add(0, na::Isometry3::identity(), floor, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

    let mut plane_transform = na::Transform3::identity();
    plane_transform[(0, 0)] = 100.;
    plane_transform[(1, 1)] = 100.;
    let floor_world_trans = plane_transform *
        na::Translation3::from_vector([0.0, 0.0, -0.5].into());

    let floor_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::World>::from_data(
            device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            Some(queue.family()),
            vs::ty::World { world: floor_world_trans.unwrap().into() },
        ).expect("failed to create buffer");

    let floor_set =
        Arc::new(
            vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(pipeline.clone(), 0)
                .add_buffer(floor_uniform_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

    let ceil_world_trans = plane_transform * na::Translation3::from_vector([0.0, 0.0, 1.5].into());

    let ceil_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::World>::from_data(
            device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            Some(queue.family()),
            vs::ty::World { world: ceil_world_trans.unwrap().into() },
        ).expect("failed to create buffer");

    let ceil_set =
        Arc::new(
            vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(pipeline.clone(), 0)
                .add_buffer(ceil_uniform_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

    let wall_shape = Cuboid::new(Vector3::new(0.5f32, 0.5, 0.5));
    let wall = ShapeHandle3::new(wall_shape);
    let wall_pos = na::Isometry3::new(Vector3::new(4.0, 0.0, 0.0), na::zero());
    world.deferred_add(1, wall_pos, wall, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

    let wall_world_trans: na::Transform3<f32> = na::Similarity3::from_isometry(wall_pos, 0.5f32)
        .to_superset();

    let wall_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::World>::from_data(
            device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            Some(queue.family()),
            vs::ty::World { world: wall_world_trans.unwrap().into() },
        ).expect("failed to create buffer");

    let wall_set =
        Arc::new(
            vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(pipeline.clone(), 0)
                .add_buffer(wall_uniform_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

    let mut x = 0.;
    let mut y = 0.;

    let mut directions = vec!();

    world.update();

    loop {
        // Poll events
        let mut done = false;
        events_loop.poll_events(|ev| match ev {
            // TODO: get mouse from axis and check if there are differences because of acceleration
            winit::Event::WindowEvent { event: winit::WindowEvent::MouseMoved { position: (dx, dy), .. }, .. } => {
                window.window().set_cursor_position(width as i32 / 2, height as i32 / 2).unwrap();
                x += (dx as f32 - width as f32 / 2.0) / 5000.0;
                y += (dy as f32 - height as f32 / 2.0) / 5000.0;
                y = y.min(::std::f32::consts::FRAC_PI_2).max(-::std::f32::consts::FRAC_PI_2);
                println!("{}: {}", x, y);
            },
            winit::Event::WindowEvent { event: winit::WindowEvent::KeyboardInput { input, .. }, .. } => {
                let direction = match input.scancode {
                    25 => Some(Direction::Forward),
                    38 => Some(Direction::Left),
                    39 => Some(Direction::Backward),
                    40 => Some(Direction::Right),
                    _ => None,
                };
                if let Some(direction) = direction {
                    directions.retain(|&elt| elt != direction);
                    if let winit::ElementState::Pressed = input.state {
                        directions.push(direction);
                    }
                }
            },
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
            winit::Event::WindowEvent { event: winit::WindowEvent::Focused(true), .. } => {
                window.window().set_cursor_state(winit::CursorState::Normal).unwrap();
                window.window().set_cursor_state(winit::CursorState::Grab).unwrap();
            },
            _ => (),
        });
        if done {
            return;
        }

        let mut move_vector = Vector3::new(0.0, 0.0, 0.0);
        for &direction in &directions {
            match direction {
                Direction::Forward => move_vector[0] = 1.0,
                Direction::Backward => move_vector[0] = -1.0,
                Direction::Left => move_vector[1] = 1.0,
                Direction::Right => move_vector[1] = -1.0,
            }
        }
        if move_vector != na::zero() {
            let mut move_vector = 0.01f32 * move_vector.normalize();
            move_vector = na::Rotation3::new(Vector3::new(0.0, 0.0, -x)) * move_vector;

            let pos = {
                let mut character = world.collision_object(0).unwrap();
                na::Translation3::from_vector(move_vector) * character.position
            };
            world.deferred_set_position(0, pos);
        }

        // Update world
        world.update();

        {
            let mut buffer = view_uniform_buffer.write().unwrap();
            let pos = world.collision_object(0).unwrap().position;
            let dir = na::Rotation3::new(Vector3::new(0.0, 0.0, -x)) * na::Rotation3::new(Vector3::new(0.0, -y, 0.0)) * Vector3::new(1.0, 0.0, 0.0);
            let view_matrix = {
                let i: na::Transform3<f32> =
                    na::Similarity3::look_at_rh(
                        &na::PointBase::from_coordinates(pos.translation.vector.into()),
                        &na::PointBase::from_coordinates(Vector3::from(pos.translation.vector) + dir),
                        &[0.0, 0.0, 1.0].into(), // FIXME: this will result in NaN if y is PI/2 isn't it ?
                        0.1,
                        ).to_superset();
                i.unwrap()
            };
            buffer.view = view_matrix.into();
        }

        // Render world
        previous_frame_end.cleanup_finished();

        let (image_num, acquire_future) = swapchain::acquire_next_image(swapchain.clone(), None)
            .unwrap();
        let mut command_buffer_builder =
            AutoCommandBufferBuilder::new(device.clone(), queue.family())
                .unwrap()
                .begin_render_pass(
                    framebuffers[image_num].clone(),
                    false,
                    vec![1f32.into()],
                )
                .unwrap();

        command_buffer_builder = command_buffer_builder
            .draw(
                pipeline.clone(),
                DynamicState::none(),
                cuboid_vertex_buffer.clone(),
                (view_set.clone(), wall_set.clone()),
                fs::ty::Group { group: 1 },
            )
            .unwrap();

        // command_buffer_builder = command_buffer_builder
        //     .draw(
        //         pipeline.clone(),
        //         DynamicState::none(),
        //         cuboid_vertex_buffer.clone(),
        //         (view_set.clone(), ceil_set.clone()),
        //         fs::ty::Group { group: 1 },
        //     )
        //     .unwrap();

        // command_buffer_builder = command_buffer_builder
        //     .draw(
        //         pipeline.clone(),
        //         DynamicState::none(),
        //         cuboid_vertex_buffer.clone(),
        //         (view_set.clone(), floor_set.clone()),
        //         fs::ty::Group { group: 1 },
        //     )
        //     .unwrap();

        command_buffer_builder = command_buffer_builder
            .next_subpass(false)
            .unwrap();

        let command_buffer = command_buffer_builder
            .end_render_pass()
            .unwrap()
            .build()
            .unwrap();

        let future = previous_frame_end
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
            .then_signal_fence_and_flush()
            .unwrap();
        previous_frame_end = Box::new(future) as Box<_>;

        // Sleep
        fps.tick()
    }
}
