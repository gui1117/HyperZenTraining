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
                    // TODO maybe store this in body directly
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

                    let player_momentum = (&players, &mut momentums).join().next().unwrap().1;
                    let mut move_vector: ::na::Vector3<f32> = ::na::zero();
                    if self.directions.is_empty() {
                        player_momentum.direction = ::na::zero();
                    } else {
                        for &direction in &self.directions {
                            match direction {
                                ::util::Direction::Forward => move_vector[0] = 1.0,
                                ::util::Direction::Backward => move_vector[0] = -1.0,
                                ::util::Direction::Left => move_vector[1] = 1.0,
                                ::util::Direction::Right => move_vector[1] = -1.0,
                            }
                        }
                        move_vector = (::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, - control.pointer[0])) * move_vector).normalize();
                        player_momentum.direction = move_vector;
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
        ::specs::ReadStorage<'a, ::component::Momentum>,
        ::specs::WriteStorage<'a, ::component::PhysicRigidBodyHandle>,
        ::specs::Fetch<'a, ::resource::Config>,
        ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    );

    fn run(&mut self, (momentums, mut bodies, config, mut physic_world): Self::SystemData) {
        for (momentum, body) in (&momentums, &mut bodies).join() {
            let mut body = body.get_mut(&mut physic_world);
            let lin_vel = body.lin_vel();
            body.append_lin_force(momentum.force*momentum.direction);
            body.append_lin_force(-momentum.damping*lin_vel);
            // TODO gravity if not touching floor
            // body.append_lin_force(10.0*::na::Vector3::new(0.0,0.0,-1.0));
        }
        for _ in 0..5 {
            physic_world.0.step(config.dt/5.);
        }
        for (momentum, body) in (&momentums, &mut bodies).join() {
            let mut body = body.get_mut(&mut physic_world);
            body.clear_forces();
            body.set_ang_acc_scale(::na::zero());
            body.set_ang_vel(::na::zero());

            let mut pos = body.position().clone();
            pos = ::na::Isometry3::new(::na::Vector3::new(pos.translation.vector[0], pos.translation.vector[1], 0.5), ::na::Vector3::x()*::std::f32::consts::FRAC_PI_2);
            body.set_transformation(pos);
        }
    }
}

pub struct DrawSystem;

impl<'a> ::specs::System<'a> for DrawSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::StaticDraw>,
        ::specs::ReadStorage<'a, ::component::DynamicDraw>,
        ::specs::ReadStorage<'a, ::component::PhysicRigidBodyHandle>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::FetchMut<'a, ::resource::Rendering>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::Control>,
        ::specs::Fetch<'a, ::resource::Graphics>,
    );

    fn run(&mut self, (static_draws, dynamic_draws, bodies, players, mut rendering, physic_world, control, graphics): Self::SystemData) {
        // Compute view uniform
        let view_uniform_buffer_subbuffer = {
            let player = (&players, &bodies).join().next().unwrap().1.get(&physic_world);
            let pos = player.position();
            let dir = ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -control.pointer[0])) *
                ::na::Rotation3::new(::na::Vector3::new(0.0, -control.pointer[1], 0.0)) *
                ::na::Vector3::new(1.0, 0.0, 0.0);

            let view_matrix = {
                let i: ::na::Transform3<f32> =
                    ::na::Similarity3::look_at_rh(
                        &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector)),
                        &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector) + dir),
                        &[0.0, 0.0, 1.0].into(), // FIXME: this will result in NaN if y is PI/2 isn't it ?
                        // &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector) + ::na::Vector3::new(0.0, 0.0, -10.0)),
                        // &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector)),
                        // &[-1.0, 0.0, 0.0].into(),
                        1.0,
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
            ).add_buffer(view_uniform_buffer_subbuffer)
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
                    graphics.plane_vertex_buffer.clone(),
                    (view_set.clone(), static_draw.set.clone()),
                    ::graphics::shader::fs::ty::Group { group: static_draw.constant },
                )
                .unwrap();
        }

        for dynamic_draw in dynamic_draws.join() {
            let world_trans_subbuffer = dynamic_draw.uniform_buffer_pool.next(dynamic_draw.world_trans);
            let dynamic_draw_set = Arc::new(
                ::vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
                    graphics.pipeline.clone(),
                    0,
                ).add_buffer(world_trans_subbuffer)
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            command_buffer_builder = command_buffer_builder
                .draw(
                    graphics.pipeline.clone(),
                    ::vulkano::command_buffer::DynamicState::none(),
                    graphics.plane_vertex_buffer.clone(),
                    (view_set.clone(), dynamic_draw_set),
                    ::graphics::shader::fs::ty::Group { group: dynamic_draw.constant },
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

// TODO
// pub struct UpdateDynamicDrawSystem;

// impl<'a> ::specs::System<'a> for UpdateDynamicDrawSystem {
//     type SystemData = (
//         ::specs::ReadStorage<'a, ::component::ColBody>,
//         ::specs::WriteStorage<'a, ::component::DynamicDraw>,
//         ::specs::Fetch<'a, ::resource::ColWorld>,
//         ::specs::Entities<'a>,
//     );

//     fn run(&mut self, (col_bodies, mut dynamic_draws, col_world, entities): Self::SystemData) {
//         for (dynamic_draw, _, entity) in (&mut dynamic_draws, &col_bodies, &*entities).join() {
//             let pos = col_world.collision_object(entity.id() as usize).unwrap().position;

//             // TODO second arg !
//             let trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 0.1)
//                 .to_superset();
//             dynamic_draw.world_trans = ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
//         }
//     }
// }
