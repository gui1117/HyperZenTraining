use specs::Join;
use alga::general::SubsetOf;
use std::sync::Arc;

// TODO: get mouse from axis and check if there are differences because of acceleration
pub struct ControlSystem {
    directions: Vec<::util::Direction>,
}

impl ControlSystem {
    pub fn new() -> Self {
        ControlSystem {
            directions: vec!(),
        }
    }
}

impl<'a> ::specs::System<'a> for ControlSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::Fetch<'a, ::resource::WinitEvents>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::Fetch<'a, ::resource::Config>,
        ::specs::FetchMut<'a, ::resource::Control>,
    );

    fn run(&mut self, (players, mut momentums, events, graphics, config, mut control): Self::SystemData) {
        for ev in events.iter() {
            match *ev {
                ::winit::Event::WindowEvent {
                    event: ::winit::WindowEvent::MouseMoved { position: (dx, dy), .. }, ..
                } => {
                    control.pointer[0] += (dx as f32 - graphics.width as f32 / 2.0) / config.mouse_sensibility;
                    control.pointer[1] += (dy as f32 - graphics.height as f32 / 2.0) / config.mouse_sensibility;
                    control.pointer[1] = control.pointer[1].min(::std::f32::consts::FRAC_PI_2).max(
                        -::std::f32::consts::FRAC_PI_2,
                    );
                },
                ::winit::Event::WindowEvent {
                    event: ::winit::WindowEvent::KeyboardInput { input, .. }, ..
                } => {
                    let direction = match input.scancode {
                        25 => Some(::util::Direction::Forward),
                        38 => Some(::util::Direction::Left),
                        39 => Some(::util::Direction::Backward),
                        40 => Some(::util::Direction::Right),
                        _ => None,
                    };
                    if let Some(direction) = direction {
                        self.directions.retain(|&elt| elt != direction);
                        if let ::winit::ElementState::Pressed = input.state {
                            self.directions.push(direction);
                        }
                    }

                    for (_, momentum) in (&players, &mut momentums).join() {
                        let mut move_vector = ::na::Vector3::new(0.0, 0.0, 0.0);
                        for &direction in &self.directions {
                            match direction {
                                ::util::Direction::Forward => move_vector[0] = 1.0,
                                ::util::Direction::Backward => move_vector[0] = -1.0,
                                ::util::Direction::Left => move_vector[1] = 1.0,
                                ::util::Direction::Right => move_vector[1] = -1.0,
                            }
                        }
                        momentum.force_direction = move_vector.normalize();
                    }
                },
                _ => (),
            }
        }
    }
}

pub struct PhysicSystem;

impl<'a> ::specs::System<'a> for PhysicSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::ColBody>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::Fetch<'a, ::resource::Config>,
        ::specs::FetchMut<'a, ::resource::ColWorld>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (col_bodies, mut momentums, config, mut col_world, entities): Self::SystemData) {
        for (_, momentum, entity) in (&col_bodies, &mut momentums, &*entities).join() {
            momentum.acceleration = (momentum.force_coefficient*momentum.force_direction - momentum.damping*momentum.velocity) / momentum.weight;
            momentum.velocity += config.dt*momentum.acceleration;
            let new_pos = {
                let col_object = col_world.collision_object(entity.id() as usize).unwrap();
                ::na::Translation3::from_vector(config.dt*momentum.velocity) * col_object.position
            };
            col_world.deferred_set_position(entity.id() as usize, new_pos);
        }

        col_world.update();
    }
}

pub struct DrawSystem;

impl<'a> ::specs::System<'a> for DrawSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::StaticDraw>,
        ::specs::ReadStorage<'a, ::component::ColBody>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::FetchMut<'a, ::resource::Rendering>,
        ::specs::Fetch<'a, ::resource::ColWorld>,
        ::specs::Fetch<'a, ::resource::Control>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (static_draws, col_bodies, players, mut rendering, col_world, control, graphics, entities): Self::SystemData) {
        let (_, _, player_entity) = (&players, &col_bodies, &*entities).join().next().unwrap();
        // Compute view uniform
        let view_uniform_buffer_subbuffer = {
            let pos = col_world.collision_object(player_entity.id() as usize).unwrap().position;
            let dir = ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -control.pointer[0])) *
                ::na::Rotation3::new(::na::Vector3::new(0.0, -control.pointer[1], 0.0)) *
                ::na::Vector3::new(1.0, 0.0, 0.0);

            let view_matrix = {
                let i: ::na::Transform3<f32> =
                    ::na::Similarity3::look_at_rh(
                        &::na::Point3::from_coordinates(pos.translation.vector.into()),
                        &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector) + dir),
                        &[0.0, 0.0, 1.0].into(), // FIXME: this will result in NaN if y is PI/2 isn't it ?
                        0.1,
                        ).to_superset();
                i.unwrap()
            };

            let proj_matrix = ::na::Perspective3::new(
                graphics.width as f32 / graphics.height as f32,
                ::std::f32::consts::FRAC_PI_3,
                0.01,
                100.0,
                ).unwrap();

            let view_uniform = ::graphics::shader::vs::ty::View {
                view: view_matrix.into(),
                proj: proj_matrix.into(),
            };

            graphics.view_uniform_buffer.next(view_uniform)
        };

        // Compute view set
        let view_set = Arc::new(
            ::vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
                graphics.pipeline.clone(),
                0,
            ).add_buffer(view_uniform_buffer_subbuffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        // Compute command
        let mut command_buffer_builder =
            ::vulkano::command_buffer::AutoCommandBufferBuilder::new(graphics.device.clone(), graphics.queue.family())
                .unwrap()
                .begin_render_pass(
                    graphics.framebuffer.clone(),
                    false,
                    vec![0u32.into(), 1f32.into()],
                )
                .unwrap();

        for static_draw in static_draws.join() {
            command_buffer_builder = command_buffer_builder
                .draw(
                    graphics.pipeline.clone(),
                    ::vulkano::command_buffer::DynamicState::none(),
                    graphics.cuboid_vertex_buffer.clone(),
                    (view_set.clone(), static_draw.set.clone()),
                    ::graphics::shader::fs::ty::Group { group: static_draw.constant },
                )
                .unwrap();
        }

        rendering.command_buffer = Some(command_buffer_builder
            .end_render_pass()
            .unwrap()
            .build()
            .unwrap());

        // Compute second command
        rendering.second_command_buffer = Some(::vulkano::command_buffer::AutoCommandBufferBuilder::new(graphics.device.clone(), graphics.queue.family()).unwrap()
            .begin_render_pass(graphics.second_framebuffers[rendering.image_num.take().unwrap()].clone(), false, vec!())
            .unwrap()
            .draw(graphics.second_pipeline.clone(), ::vulkano::command_buffer::DynamicState::none(), graphics.fullscreen_vertex_buffer.clone(), graphics.tmp_image_set.clone(), ())
            .unwrap()
            .end_render_pass()
            .unwrap()
            .build().unwrap());
    }
}
