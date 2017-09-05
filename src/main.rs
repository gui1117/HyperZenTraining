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

mod util;
mod graphics;

use vulkano_win::VkSurfaceBuild;

use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::sampler::{Sampler, Filter, UnnormalizedSamplerAddressMode};
use vulkano::swapchain;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use ncollide::world::{CollisionWorld, CollisionGroups, GeometricQueryType};
use ncollide::shape::{Cylinder, Cuboid, ShapeHandle3};
use alga::general::SubsetOf;

use util::Direction;

use std::sync::Arc;

fn main() {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        let info = app_info_from_cargo_toml!();
        ::vulkano::instance::Instance::new(Some(&info), &extensions, None)
            .expect("failed to create Vulkan instance")
    };

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();

    window.window().set_cursor(winit::MouseCursor::NoneCursor);
    window
        .window()
        .set_cursor_state(winit::CursorState::Grab)
        .unwrap();

    let graphics = graphics::Graphics::new(&window);

    let width = graphics.images[0].dimensions()[0];
    let height = graphics.images[0].dimensions()[1];

    let mut previous_frame_end = Box::new(now(graphics.device.clone())) as Box<GpuFuture>;

    let mut fps = fps_clock::FpsClock::new(60);

    let mut world = CollisionWorld::new(0.02, false);

    let character = ShapeHandle3::new(Cylinder::new(0.5f32, 0.3));
    let character_pos = na::Isometry3::new(na::Vector3::new(-1.0, 0.0, 0.0), na::Vector3::z());
    let character_groups = CollisionGroups::new();
    world.deferred_add(0, character_pos, character, character_groups, GeometricQueryType::Contacts(0.0), ());

    let view_uniform_buffer =
        vulkano::buffer::cpu_pool::CpuBufferPool::<graphics::shader::vs::ty::View>::new(
            graphics.device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
        );

    let mut wall_kind_groups = CollisionGroups::new();
    wall_kind_groups.set_membership(&[2]);
    wall_kind_groups.set_blacklist(&[2]);

    // let floor = ShapeHandle3::new(Plane::new(na::Vector3::new(0.0, 0.0, 1.0)));
    // world.deferred_add(0, na::Isometry3::identity(), floor, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

    let mut plane_transform = na::Transform3::identity();
    plane_transform[(0, 0)] = 10.;
    plane_transform[(1, 1)] = 10.;
    let floor_world_trans = plane_transform *
        na::Translation3::from_vector([0.0, 0.0, -10.5].into());

    let floor_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<graphics::shader::vs::ty::World>::from_data(
            graphics.device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            graphics::shader::vs::ty::World { world: floor_world_trans.unwrap().into() },
        ).expect("failed to create buffer");

    let floor_set = Arc::new(
        vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
            graphics.pipeline.clone(),
            0,
        ).add_buffer(floor_uniform_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let ceil_world_trans = plane_transform * na::Translation3::from_vector([0.0, 0.0, 1.5].into());

    let ceil_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<graphics::shader::vs::ty::World>::from_data(
            graphics.device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            graphics::shader::vs::ty::World { world: ceil_world_trans.unwrap().into() },
        ).expect("failed to create buffer");

    let ceil_set = Arc::new(
        vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
            graphics.pipeline.clone(),
            0,
        ).add_buffer(ceil_uniform_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let wall_shape = Cuboid::new(na::Vector3::new(0.5f32, 0.5, 0.5));
    let wall = ShapeHandle3::new(wall_shape);
    let wall_pos = na::Isometry3::new(na::Vector3::new(0.0, 0.0, 0.0), na::zero());
    world.deferred_add(1, wall_pos, wall, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

    let wall_world_trans: na::Transform3<f32> = na::Similarity3::from_isometry(wall_pos, 0.5f32)
        .to_superset();

    let wall_uniform_buffer =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<graphics::shader::vs::ty::World>::from_data(
            graphics.device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            graphics::shader::vs::ty::World { world: wall_world_trans.unwrap().into() },
        ).expect("failed to create buffer");

    let wall_set = Arc::new(
        vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
            graphics.pipeline.clone(),
            0,
        ).add_buffer(wall_uniform_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let wall_shape_2 = Cuboid::new(na::Vector3::new(0.5f32, 0.5, 0.5));
    let wall_2 = ShapeHandle3::new(wall_shape_2);
    let wall_pos_2 = na::Isometry3::new(na::Vector3::new(3.0, 5.0, -4.0), na::zero());
    world.deferred_add(1, wall_pos_2, wall_2, wall_kind_groups, GeometricQueryType::Contacts(0.0), ());

    let wall_world_trans_2: na::Transform3<f32> =
        na::Similarity3::from_isometry(wall_pos_2, 0.5f32).to_superset();

    let wall_uniform_buffer_2 =
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<graphics::shader::vs::ty::World>::from_data(
            graphics.device.clone(),
            vulkano::buffer::BufferUsage::uniform_buffer(),
            graphics::shader::vs::ty::World { world: wall_world_trans_2.unwrap().into() },
        ).expect("failed to create buffer");

    let wall_set_2 = Arc::new(
        vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
            graphics.pipeline.clone(),
            0,
        ).add_buffer(wall_uniform_buffer_2.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    //TODO use simple instead of persistent
    let tmp_image_set = Arc::new(
        vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
            graphics.second_pipeline.clone(),
            0,
        ).add_sampled_image(
            graphics.tmp_image_attachment.clone(),
            // Sampler::simple_repeat_linear_no_mipmap(graphics.device.clone()),
            Sampler::unnormalized(
                graphics.device.clone(),
                Filter::Nearest,
                UnnormalizedSamplerAddressMode::ClampToEdge,
                UnnormalizedSamplerAddressMode::ClampToEdge,
            ).unwrap(),
        )
            .unwrap()
            .build()
            .unwrap(),
    );

    let mut x = 0.0f32;
    let mut y = 0.0f32;

    let mut directions = vec![];

    world.update();

    loop {
        previous_frame_end.cleanup_finished();

        // Poll events
        let mut done = false;
        events_loop.poll_events(|ev| match ev {
            // TODO: get mouse from axis and check if there are differences because of acceleration
            winit::Event::WindowEvent {
                event: winit::WindowEvent::MouseMoved { position: (dx, dy), .. }, ..
            } => {
                window
                    .window()
                    .set_cursor_position(width as i32 / 2, height as i32 / 2)
                    .unwrap();
                x += (dx as f32 - width as f32 / 2.0) / 5000.0;
                y += (dy as f32 - height as f32 / 2.0) / 5000.0;
                y = y.min(::std::f32::consts::FRAC_PI_2).max(
                    -::std::f32::consts::FRAC_PI_2,
                );
            }
            winit::Event::WindowEvent {
                event: winit::WindowEvent::KeyboardInput { input, .. }, ..
            } => {
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
            }
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
            _ => (),
        });
        if done {
            return;
        }

        let mut move_vector = na::Vector3::new(0.0, 0.0, 0.0);
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
            move_vector = na::Rotation3::new(na::Vector3::new(0.0, 0.0, -x)) * move_vector;

            let pos = {
                let character = world.collision_object(0).unwrap();
                na::Translation3::from_vector(move_vector) * character.position
            };
            world.deferred_set_position(0, pos);
        }

        // Update world
        world.update();

        let view_uniform_buffer_subbuffer = {
            let pos = world.collision_object(0).unwrap().position;
            let dir = na::Rotation3::new(na::Vector3::new(0.0, 0.0, -x)) *
                na::Rotation3::new(na::Vector3::new(0.0, -y, 0.0)) *
                na::Vector3::new(1.0, 0.0, 0.0);

            let view_matrix = {
                let i: na::Transform3<f32> =
                    na::Similarity3::look_at_rh(
                        &na::Point3::from_coordinates(pos.translation.vector.into()),
                        &na::Point3::from_coordinates(na::Vector3::from(pos.translation.vector) + dir),
                        &[0.0, 0.0, 1.0].into(), // FIXME: this will result in NaN if y is PI/2 isn't it ?
                        0.1,
                        ).to_superset();
                i.unwrap()
            };

            let proj_matrix = na::Perspective3::new(
                width as f32 / height as f32,
                ::std::f32::consts::FRAC_PI_3,
                0.01,
                100.0,
            ).unwrap();

            let view_uniform = graphics::shader::vs::ty::View {
                view: view_matrix.into(),
                proj: proj_matrix.into(),
            };

            view_uniform_buffer.next(view_uniform)
        };


        let view_set = Arc::new(
            vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
                graphics.pipeline.clone(),
                0,
            ).add_buffer(view_uniform_buffer_subbuffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        // Render world
        let (image_num, acquire_future) =
            swapchain::acquire_next_image(graphics.swapchain.clone(), None).unwrap();
        let mut command_buffer_builder =
            AutoCommandBufferBuilder::new(graphics.device.clone(), graphics.queue.family())
                .unwrap()
                .begin_render_pass(
                    graphics.framebuffer.clone(),
                    false,
                    vec![0u32.into(), 1f32.into()],
                )
                .unwrap();

        command_buffer_builder = command_buffer_builder
            .draw(
                graphics.pipeline.clone(),
                DynamicState::none(),
                graphics.cuboid_vertex_buffer.clone(),
                (view_set.clone(), wall_set.clone()),
                graphics::shader::fs::ty::Group { group: 1 },
            )
            .unwrap();

        command_buffer_builder = command_buffer_builder
            .draw(
                graphics.pipeline.clone(),
                DynamicState::none(),
                graphics.cuboid_vertex_buffer.clone(),
                (view_set.clone(), wall_set_2.clone()),
                graphics::shader::fs::ty::Group { group: 4 },
            )
            .unwrap();

        command_buffer_builder = command_buffer_builder
            .draw(
                graphics.pipeline.clone(),
                DynamicState::none(),
                graphics.cuboid_vertex_buffer.clone(),
                (view_set.clone(), ceil_set.clone()),
                graphics::shader::fs::ty::Group { group: 2 },
            )
            .unwrap();

        command_buffer_builder = command_buffer_builder
            .draw(
                graphics.pipeline.clone(),
                DynamicState::none(),
                graphics.cuboid_vertex_buffer.clone(),
                (view_set.clone(), floor_set.clone()),
                graphics::shader::fs::ty::Group { group: 3 },
            )
            .unwrap();

        let command_buffer = command_buffer_builder
            .end_render_pass()
            .unwrap()
            .build()
            .unwrap();

        let second_command_buffer = AutoCommandBufferBuilder::new(graphics.device.clone(), graphics.queue.family()).unwrap()
            .begin_render_pass(graphics.second_framebuffers[image_num].clone(), false, vec!())
            .unwrap()
            .draw(graphics.second_pipeline.clone(), DynamicState::none(), graphics.fullscreen_vertex_buffer.clone(), tmp_image_set.clone(), ())
            .unwrap()
            .end_render_pass()
            .unwrap()
            .build().unwrap();

        // TODO submit first pass before call image from swapchain
        let future = previous_frame_end
            .join(acquire_future)
            .then_execute(graphics.queue.clone(), command_buffer)
            .unwrap()
            .then_execute(graphics.queue.clone(), second_command_buffer)
            .unwrap()
            .then_swapchain_present(
                graphics.queue.clone(),
                graphics.swapchain.clone(),
                image_num,
            )
            .then_signal_fence_and_flush()
            .unwrap();
        previous_frame_end = Box::new(future) as Box<_>;

        // Sleep
        fps.tick()
    }
}
