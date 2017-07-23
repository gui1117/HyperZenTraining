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
use vulkano::buffer::ImmutableBuffer;
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
use vulkano::swapchain::ColorSpace;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use na::{Vector3, Point3};
use ncollide::world::{CollisionWorld, CollisionGroups, GeometricQueryType, CollisionObject3};
use ncollide::narrow_phase::{ProximityHandler, ContactHandler, ContactAlgorithm3};
use ncollide::shape::{Plane, Ball, Cylinder, Cuboid, ShapeHandle3};
use ncollide::query::Proximity;
use ncollide::transformation::ToTriMesh;
use alga::general::SubsetOf;
use alga::general::SupersetOf;

use std::iter;
use std::sync::Arc;
use std::time::Duration;
use std::cell::Cell;

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
    mat4 worldview = view.view * world.world;
    // gl_Position = view.proj * worldview * vec4(position, 1.0);
    gl_Position = worldview * vec4(position, 1.0);
}
"]
    struct Dummy;
}

mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
#version 450

layout(location = 0) out vec4 f_color;

layout(push_constant) uniform Group {
    uint group;
} group;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"]
    struct Dummy;
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
            Vertex { position: [-1.0f32, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [1.0, -1.0, -1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [1.0, -1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, -1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.0] },
            Vertex { position: [-1.0, 1.0, 1.0] },
            Vertex { position: [1.0, -1.0, 1.] },
        ].iter()
            .cloned(),
    ).expect("failed to create buffer");

    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let render_pass = Arc::new(
        single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
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
        pass: {
            color: [color],
            depth_stencil: {depth}
        }
    ).unwrap(),
    );

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .viewports(iter::once(Viewport {
                origin: [0.0, 0.0],
                depth_range: 0.0..1.0,
                dimensions: [
                    images[0].dimensions()[0] as f32,
                    images[0].dimensions()[1] as f32,
                ],
            }))
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let framebuffers = images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .add(depth_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;

    let mut fps = fps_clock::FpsClock::new(30);

    let mut world = CollisionWorld::new(0.02, false);

    let character = ShapeHandle3::new(Cylinder::new(0.5f32, 0.3));
    let character_pos = na::Isometry3::new(Vector3::new(0.0, 0.0, 0.5), na::zero());
    let mut character_groups = CollisionGroups::new();
    world.deferred_add(0, character_pos, character, character_groups, GeometricQueryType::Contacts(0.0), ());

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
    let wall_pos = na::Isometry3::new(Vector3::new(10.0, 0.0, 0.5), na::zero());
    world.deferred_add(0, wall_pos, wall, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

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

    let view = {
        let i: na::Transform3<f32> =
            na::Similarity3::look_at_lh(
                &na::PointBase::from_coordinates([0.0, 0.0, 0.5].into()),
                &na::PointBase::from_coordinates([1.0, 0.0, 0.5].into()),
                &[0.0, 0.0, 1.0].into(),
                0.01,
                ).to_superset();
        i.unwrap()
    };
    let view_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::View>::from_data(
            device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            Some(queue.family()),
            vs::ty::View {
                view: view.into(),
                proj: na::Perspective3::new(
                    images[0].dimensions()[1] as f32 / images[0].dimensions()[0] as f32,
                    ::std::f32::consts::FRAC_PI_3,
                    0.01,
                    100.0,
                ).unwrap()
                    .into(),
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

    let p = [
            na::Vector4::new(0.0f32, -1.0, -1.0, 1.0),
            na::Vector4::new(0.0f32, 1.0, 1.0, 1.0),
            na::Vector4::new(0.0f32, -1.0, 1.0, 1.0),
    ];
    println!("{:#?}", view);
    for p in p.iter() {
    println!("{:#?}", view * wall_world_trans.unwrap() * p);
    }

    loop {
        // Poll events
        let mut done = false;
        events_loop.poll_events(|ev| match ev {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
            _ => (),
        });
        if done {
            return;
        }

        // Update world
        // world.update();

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
                    vec![[0.0, 0.0, 1.0, 1.0].into(), 1f32.into()],
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
