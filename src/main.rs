extern crate winit;
extern crate vulkano_win;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate fps_clock;
extern crate alga;
extern crate specs;
extern crate nalgebra as na;
extern crate ncollide;

mod util;
mod graphics;
mod entity;
mod component;
mod system;
mod resource;

use vulkano_win::VkSurfaceBuild;

use vulkano::swapchain;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use util::Direction;

use std::sync::Arc;

pub type ColGroup = ::ncollide::world::CollisionGroups;
pub type ColPoint = na::Point<f32, na::U3>;
pub type ColPosition = na::Isometry<f32, na::U3, na::Unit<na::Quaternion<f32>>>;
pub type ColShape = ::ncollide::shape::ShapeHandle<ColPoint, ColPosition>;
pub type ColWorld = ::ncollide::world::CollisionWorld<na::Point<f32, na::U3>, ColPosition, ()>;

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

    let mut previous_frame_end = Box::new(now(graphics.data.device.clone())) as Box<GpuFuture>;

    let mut fps = fps_clock::FpsClock::new(60);

    let mut world = specs::World::new();
    world.register::<::component::Player>();
    world.register::<::component::ColBody>();
    world.register::<::component::StaticDraw>();
    world.add_resource(graphics.data.clone());
    world.add_resource(ColWorld::new(0.02, false));
    world.add_resource(::resource::Control::new());
    world.add_resource(::resource::Rendering::new());

    ::entity::create_player(&mut world);
    ::entity::create_wall(&mut world, [4.0, 0.0]);

    world.maintain();
    world.write_resource::<::resource::ColWorld>().update();


    let mut draw_dispatcher = ::specs::DispatcherBuilder::new()
        .add(::system::DrawSystem, "draw_system", &[])
        .build();

    loop {
        previous_frame_end.cleanup_finished();

        // Poll events
        let mut done = false;
        events_loop.poll_events(|ev| match ev {
            // TODO: get mouse from axis and check if there are differences because of acceleration
            winit::Event::WindowEvent {
                event: winit::WindowEvent::MouseMoved { position: (dx, dy), .. }, ..
            } => {
                let mut control = world.write_resource::<::resource::Control>();
                window
                    .window()
                    .set_cursor_position(graphics.data.width as i32 / 2, graphics.data.height as i32 / 2)
                    .unwrap();
                control.pointer[0] += (dx as f32 - graphics.data.width as f32 / 2.0) / 5000.0;
                control.pointer[1] += (dy as f32 - graphics.data.height as f32 / 2.0) / 5000.0;
                control.pointer[1] = control.pointer[1].min(::std::f32::consts::FRAC_PI_2).max(
                    -::std::f32::consts::FRAC_PI_2,
                );
            }
            winit::Event::WindowEvent {
                event: winit::WindowEvent::KeyboardInput { input, .. }, ..
            } => {
                let mut control = world.write_resource::<::resource::Control>();
                let direction = match input.scancode {
                    25 => Some(Direction::Forward),
                    38 => Some(Direction::Left),
                    39 => Some(Direction::Backward),
                    40 => Some(Direction::Right),
                    _ => None,
                };
                if let Some(direction) = direction {
                    control.directions.retain(|&elt| elt != direction);
                    if let winit::ElementState::Pressed = input.state {
                        control.directions.push(direction);
                    }
                }
            }
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
            _ => (),
        });
        if done {
            return;
        }

        // let mut move_vector = na::Vector3::new(0.0, 0.0, 0.0);
        // for &direction in &directions {
        //     match direction {
        //         Direction::Forward => move_vector[0] = 1.0,
        //         Direction::Backward => move_vector[0] = -1.0,
        //         Direction::Left => move_vector[1] = 1.0,
        //         Direction::Right => move_vector[1] = -1.0,
        //     }
        // }
        // if move_vector != na::zero() {
        //     let mut move_vector = 0.01f32 * move_vector.normalize();
        //     move_vector = na::Rotation3::new(na::Vector3::new(0.0, 0.0, -x)) * move_vector;

        //     let pos = {
        //         let character = world.collision_object(0).unwrap();
        //         na::Translation3::from_vector(move_vector) * character.position
        //     };
        //     world.deferred_set_position(0, pos);
        // }

        // TODO update collision world
        // colworld.update();
        world.write_resource::<::resource::ColWorld>().update();
        world.maintain();

        // Render world
        let (image_num, acquire_future) =
            swapchain::acquire_next_image(graphics.data.swapchain.clone(), None).unwrap();
        world.write_resource::<::resource::Rendering>().image_num = Some(image_num);

        draw_dispatcher.dispatch(&mut world.res);

        let (command_buffer, second_command_buffer) = {
            let mut rendering = world.write_resource::<::resource::Rendering>();
            (rendering.command_buffer.take().unwrap(), rendering.second_command_buffer.take().unwrap())
        };

        let future = previous_frame_end
            .then_execute(graphics.data.queue.clone(), command_buffer)
            .unwrap()
            .join(acquire_future)
            .then_execute(graphics.data.queue.clone(), second_command_buffer)
            .unwrap()
            .then_swapchain_present(
                graphics.data.queue.clone(),
                graphics.data.swapchain.clone(),
                image_num,
            )
            .then_signal_fence_and_flush()
            .unwrap();
        previous_frame_end = Box::new(future) as Box<_>;

        // Sleep
        fps.tick();
    }
}
