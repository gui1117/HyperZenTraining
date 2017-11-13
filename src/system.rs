use winit::{Event, WindowEvent, ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode,
            TouchPhase, DeviceEvent};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::pipeline::viewport::Viewport;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use nphysics::object::WorldObject;
use util::Direction;
use specs::Join;
use alga::general::SubsetOf;
use util::{high_byte, low_byte};
use rand::distributions::{IndependentSample, Range};

use std::sync::Arc;
use std::cell::RefCell;

pub struct MenuControlSystem {
    mouse_down: [bool; 5],
}

impl MenuControlSystem {
    pub fn new() -> Self {
        MenuControlSystem { mouse_down: [false; 5] }
    }
}

impl<'a> ::specs::System<'a> for MenuControlSystem {
    type SystemData = (::specs::Fetch<'a, ::resource::MenuEvents>,
     ::specs::FetchMut<'a, ::resource::ImGui>);

    fn run(&mut self, (events, mut imgui): Self::SystemData) {
        for ev in events.0.iter() {
            match *ev {
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. }, ..
                } => {
                    match button {
                        MouseButton::Left => self.mouse_down[0] = state == ElementState::Pressed,
                        MouseButton::Right => self.mouse_down[1] = state == ElementState::Pressed,
                        MouseButton::Middle => self.mouse_down[2] = state == ElementState::Pressed,
                        MouseButton::Other(0) => {
                            self.mouse_down[3] = state == ElementState::Pressed
                        }
                        MouseButton::Other(1) => {
                            self.mouse_down[4] = state == ElementState::Pressed
                        }
                        MouseButton::Other(_) => (),
                    }
                    imgui.set_mouse_down(&self.mouse_down);
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseMoved { position: (x, y), .. }, ..
                } => imgui.set_mouse_pos(x as f32, y as f32),
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    let pressed = input.state == ElementState::Pressed;
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Tab) => imgui.set_key(0, pressed),
                        Some(VirtualKeyCode::Left) => imgui.set_key(1, pressed),
                        Some(VirtualKeyCode::Right) => imgui.set_key(2, pressed),
                        Some(VirtualKeyCode::Up) => imgui.set_key(3, pressed),
                        Some(VirtualKeyCode::Down) => imgui.set_key(4, pressed),
                        Some(VirtualKeyCode::PageUp) => imgui.set_key(5, pressed),
                        Some(VirtualKeyCode::PageDown) => imgui.set_key(6, pressed),
                        Some(VirtualKeyCode::Home) => imgui.set_key(7, pressed),
                        Some(VirtualKeyCode::End) => imgui.set_key(8, pressed),
                        Some(VirtualKeyCode::Delete) => imgui.set_key(9, pressed),
                        Some(VirtualKeyCode::Back) => imgui.set_key(10, pressed),
                        Some(VirtualKeyCode::Return) => imgui.set_key(11, pressed),
                        Some(VirtualKeyCode::Escape) => imgui.set_key(12, pressed),
                        Some(VirtualKeyCode::A) => imgui.set_key(13, pressed),
                        Some(VirtualKeyCode::C) => imgui.set_key(14, pressed),
                        Some(VirtualKeyCode::V) => imgui.set_key(15, pressed),
                        Some(VirtualKeyCode::X) => imgui.set_key(16, pressed),
                        Some(VirtualKeyCode::Y) => imgui.set_key(17, pressed),
                        Some(VirtualKeyCode::Z) => imgui.set_key(18, pressed),
                        Some(VirtualKeyCode::LControl) |
                        Some(VirtualKeyCode::RControl) => imgui.set_key_ctrl(pressed),
                        Some(VirtualKeyCode::LShift) |
                        Some(VirtualKeyCode::RShift) => imgui.set_key_shift(pressed),
                        Some(VirtualKeyCode::LAlt) |
                        Some(VirtualKeyCode::RAlt) => imgui.set_key_alt(pressed),
                        Some(VirtualKeyCode::LWin) |
                        Some(VirtualKeyCode::RWin) => imgui.set_key_super(pressed),
                        _ => (),
                    }

                }
                Event::WindowEvent {
                    event: WindowEvent::MouseWheel {
                        delta,
                        phase: TouchPhase::Moved,
                        ..
                    },
                    ..
                } => {
                    // TODO: does both are send ? does it depend of computer
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => imgui.set_mouse_wheel(y),
                        MouseScrollDelta::PixelDelta(_, y) => imgui.set_mouse_wheel(y),
                    }
                }
                Event::WindowEvent { event: WindowEvent::ReceivedCharacter(c), .. } => {
                    imgui.add_input_character(c)
                }
                _ => (),
            }
        }
    }
}

pub struct PlayerControlSystem {
    directions: Vec<::util::Direction>,
    pointer: [f32; 2],
}

impl PlayerControlSystem {
    pub fn new() -> Self {
        PlayerControlSystem {
            directions: vec![],
            pointer: [0.0, 0.0],
        }
    }
}

impl<'a> ::specs::System<'a> for PlayerControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::WriteStorage<'a, ::component::Aim>,
     ::specs::WriteStorage<'a, ::component::Shooter>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::GameEvents>,
     ::specs::Fetch<'a, ::resource::Config>);

    fn run(
        &mut self,
        (players, mut aims, mut shooters, mut momentums, events, config): Self::SystemData,
    ) {
        let (_, player_aim, player_shooter, player_momentum) =
            (&players, &mut aims, &mut shooters, &mut momentums)
                .join()
                .next()
                .unwrap();
        for ev in events.0.iter() {
            match *ev {
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    },
                    ..
                } => {
                    match state {
                        ElementState::Pressed => player_shooter.set_shoot(true),
                        ElementState::Released => player_shooter.set_shoot(false),
                    }
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 0, value: dx }, ..
                } => {
                    self.pointer[0] += dx as f32 * config.mouse_sensibility();
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 1, value: dy }, ..
                } => {
                    self.pointer[1] += dy as f32 * config.mouse_sensibility();
                    self.pointer[1] = self.pointer[1].min(::std::f32::consts::FRAC_PI_2).max(
                        -::std::f32::consts::FRAC_PI_2,
                    );
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    let direction = match input.scancode {
                        25 => Some(Direction::Forward),
                        38 => Some(Direction::Left),
                        39 => Some(Direction::Backward),
                        40 => Some(Direction::Right),
                        _ => None,
                    };
                    if let Some(direction) = direction {
                        self.directions.retain(|&elt| elt != direction);
                        if let ElementState::Pressed = input.state {
                            self.directions.push(direction);
                        }
                    }
                }
                _ => (),
            }
        }

        player_aim.dir = ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -self.pointer[0])) *
            ::na::Rotation3::new(::na::Vector3::new(0.0, self.pointer[1], 0.0)) *
            ::na::Vector3::x();
        player_aim.x_dir = self.pointer[0];

        let mut move_vector: ::na::Vector3<f32> = ::na::zero();
        if self.directions.is_empty() {
            player_momentum.direction = ::na::zero();
        } else {
            for &direction in &self.directions {
                match direction {
                    Direction::Forward => move_vector[0] = 1.0,
                    Direction::Backward => move_vector[0] = -1.0,
                    Direction::Left => move_vector[1] = 1.0,
                    Direction::Right => move_vector[1] = -1.0,
                }
            }
            move_vector = (::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -self.pointer[0])) *
                               move_vector)
                .normalize();
            player_momentum.direction = move_vector;
        }
    }
}

pub struct AvoiderControlSystem;

impl<'a> ::specs::System<'a> for AvoiderControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Avoider>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Maze>);

    fn run(
        &mut self,
        (players, aims, bodies, mut avoiders, mut momentums, physic_world, maze): Self::SystemData,
    ) {
        let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();

        let player_pos = player_body.get(&physic_world).position().clone();

        for (avoider, momentum, body) in (&mut avoiders, &mut momentums, &bodies).join() {
            let avoider_pos = body.get(&physic_world).position().clone();

            let recompute_goal = if let Some(goal) = avoider.goal {
                (avoider_pos.translation.vector -
                     ::na::Vector3::new(
                        goal.0 as f32 + 0.5,
                        goal.1 as f32 + 0.5,
                        avoider_pos.translation.vector[2],
                    )).norm() < 0.5
            } else {
                if (avoider_pos.translation.vector - player_pos.translation.vector).norm() < 1.0 {
                    avoider.goal.take();
                    false
                } else {
                    true
                }
            };

            if recompute_goal {
                let pos = (
                    avoider_pos.translation.vector[0] as usize,
                    avoider_pos.translation.vector[1] as usize,
                );
                let goal = (
                    player_pos.translation.vector[0] as usize,
                    player_pos.translation.vector[1] as usize,
                );
                avoider.goal = maze.find_path(pos, goal).unwrap().0.get(1).cloned();
            }

            let (goal_direction, goal_coef) = {
                let goal_pos = if let Some(goal) = avoider.goal {
                    ::na::Vector3::new(
                        goal.0 as f32 + 0.5,
                        goal.1 as f32 + 0.5,
                        avoider_pos.translation.vector[2],
                    )
                } else {
                    player_pos.translation.vector
                };

                (
                    (goal_pos - avoider_pos.translation.vector).normalize(),
                    1f32,
                )
            };

            let (avoid_direction, avoid_coef) = {
                let avoider_pos_rel_player = avoider_pos.translation.vector -
                    player_pos.translation.vector;
                let avoid_vector = avoider_pos_rel_player -
                    avoider_pos_rel_player.dot(&player_aim.dir) * player_aim.dir;
                if avoid_vector.norm() != 0.0 {
                    let avoid_norm = avoid_vector.norm();
                    let avoid_direction = avoid_vector.normalize();
                    if avoid_norm > 0.5 {
                        (avoid_direction, 0f32)
                    } else {
                        // TODO: coefficent
                        (avoid_direction, 1f32) //1.0/avoid_norm)
                    }
                } else {
                    let random = ::na::Vector3::new_random();
                    (
                        (random - random.dot(&player_aim.dir) * player_aim.dir).normalize(),
                        1f32,
                        // TODO: coefficient
                    ) //1000f32)
                }
            };

            momentum.direction = (goal_coef * goal_direction + avoid_coef * avoid_direction)
                .normalize();
        }
    }
}

pub struct BouncerControlSystem;

impl<'a> ::specs::System<'a> for BouncerControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Contactor>,
     ::specs::ReadStorage<'a, ::component::Bouncer>,
     ::specs::WriteStorage<'a, ::component::Momentum>);

    fn run(&mut self, (contactors, bouncers, mut momentums): Self::SystemData) {
        for (_, momentum, contactor) in (&bouncers, &mut momentums, &contactors).join() {
            if contactor.contacts.is_empty() {
               continue;
            }

            let mut normal = ::na::Vector3::new(0.0, 0.0, 0.0);
            for &(_, ref contact) in &contactor.contacts {
                normal -= contact.depth * contact.normal;
            }
            normal.normalize_mut();
            let proj_on_normal = momentum.direction.dot(&normal) * normal;
            if proj_on_normal.dot(&normal) > 0.0 {
                momentum.direction -= 2.0 * proj_on_normal;
            }
        }
    }
}


pub struct PhysicSystem;

impl<'a> ::specs::System<'a> for PhysicSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Momentum>,
     ::specs::WriteStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Contactor>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>);

    fn run(
        &mut self,
        (player, momentums, mut bodies, mut contactors, config, mut physic_world): Self::SystemData,
    ) {
        for (momentum, body) in (&momentums, &mut bodies).join() {
            let body = body.get_mut(&mut physic_world);
            let lin_vel = body.lin_vel();
            let ang_vel = body.ang_vel();

            // TODO: use integrator to modify rigidbody
            body.clear_forces();
            body.append_lin_force(-momentum.damping * lin_vel);
            let direction_force = momentum.force * momentum.direction;
            if let Some(pnt_to_com) = momentum.pnt_to_com {
                let pnt_to_com = body.position().rotation * pnt_to_com;
                body.append_force_wrt_point(direction_force, pnt_to_com);
            } else {
                body.append_lin_force(direction_force);
            }
            body.set_ang_vel_internal(momentum.ang_damping * ang_vel);

            // TODO: gravity if not touching floor
            // body.append_lin_force(10.0*::na::Vector3::new(0.0,0.0,-1.0));
        }
        for contactor in (&mut contactors).join() {
            contactor.contacts.clear();
        }
        for _ in 0..2 {

            physic_world.step(config.dt().clone() / 2.);

            for (co1, co2, mut contact) in physic_world.collision_world().contacts() {
                match (&co1.data, &co2.data) {
                    (&WorldObject::RigidBody(co1), &WorldObject::RigidBody(co2)) => {
                        let body_1 = physic_world.rigid_body(co1);
                        let entity_1 = ::component::PhysicBody::entity(body_1);
                        let body_2 = physic_world.rigid_body(co2);
                        let entity_2 = ::component::PhysicBody::entity(body_2);

                        if let Some(contactor) = contactors.get_mut(entity_1) {
                            contactor.contacts.push((entity_2, contact.clone()));
                        }

                        if let Some(contactor) = contactors.get_mut(entity_2) {
                            contact.flip();
                            contactor.contacts.push((entity_1, contact));
                        }
                    }
                    _ => (),
                }
            }
        }
        for (_, body) in (&player, &mut bodies).join() {
            let body = body.get_mut(&mut physic_world);
            body.set_ang_acc_scale(::na::zero());
            body.set_ang_vel(::na::zero());

            let mut pos = body.position().clone();
            pos = ::na::Isometry3::new(
                ::na::Vector3::new(pos.translation.vector[0], pos.translation.vector[1], 0.5),
                ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
            );
            body.set_transformation(pos);
        }
    }
}

pub struct DrawSystem;

impl<'a> ::specs::System<'a> for DrawSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::StaticDraw>,
     ::specs::ReadStorage<'a, ::component::DynamicDraw>,
     ::specs::ReadStorage<'a, ::component::DynamicEraser>,
     ::specs::ReadStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::FetchMut<'a, ::resource::Rendering>,
     ::specs::FetchMut<'a, ::resource::ImGui>,
     ::specs::FetchMut<'a, ::resource::Graphics>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>);

    fn run(&mut self, (static_draws, dynamic_draws, dynamic_erasers, dynamic_graphics_assets, bodies, players, aims, mut rendering, mut imgui, mut graphics, config, physic_world): Self::SystemData) {
        let mut future = Vec::new();

        // Compute view uniform
        let view_uniform_buffer_subbuffer = {
            let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();

            let player_pos = player_body.get(&physic_world).position().clone();

            // IDEA: if we change -player.x here to + then it is fun
            let camera_top = if player_aim.dir[2].abs() > 0.8 {
                ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -player_aim.x_dir)) *
                    ::na::Vector3::x() * -player_aim.dir[2].signum()
            } else {
                ::na::Vector3::z()
            };

            let view_matrix = {
                let i: ::na::Transform3<f32> =
                    ::na::Similarity3::look_at_rh(
                        &::na::Point3::from_coordinates(::na::Vector3::from(player_pos.translation.vector)),
                        &::na::Point3::from_coordinates(::na::Vector3::from(player_pos.translation.vector) + player_aim.dir),
                        &camera_top.into(),
                        // &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector) + ::na::Vector3::new(0.0, 0.0, -10.0)),
                        // &::na::Point3::from_coordinates(::na::Vector3::from(pos.translation.vector)),
                        // &[-1.0, 0.0, 0.0].into(),
                        1.0,
                        ).to_superset();
                i.unwrap()
            };

            let proj_matrix = ::na::Perspective3::new(
                graphics.dim[0] as f32 / graphics.dim[1] as f32,
                ::std::f32::consts::FRAC_PI_3,
                0.01,
                100.0,
            ).unwrap();

            let view_uniform = ::graphics::shader::draw1_vs::ty::View {
                view: view_matrix.into(),
                proj: proj_matrix.into(),
            };

            graphics.view_uniform_buffer.next(view_uniform).unwrap()
        };

        let screen_dynamic_state = DynamicState {
            viewports: Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [graphics.dim[0] as f32, graphics.dim[1] as f32],
                depth_range: 0.0..1.0,
            }]),
            ..DynamicState::none()
        };

        // Compute view set
        let view_set = Arc::new(
            graphics
                .draw1_view_descriptor_set_pool
                .next()
                .add_buffer(view_uniform_buffer_subbuffer)
                .unwrap()
                .build()
                .unwrap(),
        );

        // Compute command
        let mut command_buffer_builder =
            AutoCommandBufferBuilder::primary_one_time_submit(graphics.device.clone(), graphics.queue.family())
                .unwrap()
                .begin_render_pass(
                    graphics.framebuffer.clone(),
                    false,
                    vec![0u32.into(), 0u32.into(), 1f32.into()],
                )
                .unwrap();

        for static_draw in static_draws.join() {
            command_buffer_builder = command_buffer_builder
                .draw(
                    graphics.draw1_pipeline.clone(),
                    screen_dynamic_state.clone(),
                    graphics.primitives_vertex_buffers[static_draw.primitive].clone(),
                    (view_set.clone(), static_draw.set.clone()),
                    ::graphics::shader::draw1_fs::ty::Group {
                        group_hb: high_byte(static_draw.group as u32),
                        group_lb: low_byte(static_draw.group as u32),
                        color: static_draw.color as u32,
                    },
                )
                .unwrap();
        }

        for (_, assets) in (&dynamic_draws, &dynamic_graphics_assets).join() {
            let world_trans_subbuffer = graphics
                .world_uniform_buffer
                .next(assets.world_trans)
                .unwrap();

            let dynamic_draw_set = Arc::new(
                graphics
                    .draw1_dynamic_descriptor_set_pool
                    .next()
                    .add_buffer(world_trans_subbuffer)
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            for &primitive in &assets.primitives {
                command_buffer_builder = command_buffer_builder
                    .draw(
                        graphics.draw1_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        graphics.primitives_vertex_buffers[primitive.0].clone(),
                        (view_set.clone(), dynamic_draw_set.clone()),
                        ::graphics::shader::draw1_fs::ty::Group {
                            group_hb: high_byte(primitive.1 as u32),
                            group_lb: low_byte(primitive.1 as u32),
                            color: assets.color as u32,
                        },
                    )
                    .unwrap();
            }
        }

        command_buffer_builder = command_buffer_builder.next_subpass(false).unwrap();

        for (_, assets) in (&dynamic_erasers, &dynamic_graphics_assets).join() {
            let world_trans_subbuffer = graphics
                .world_uniform_buffer
                .next(assets.world_trans)
                .unwrap();

            let dynamic_draw_set = Arc::new(
                graphics
                    .draw1_dynamic_descriptor_set_pool
                    .next()
                    .add_buffer(world_trans_subbuffer)
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            for &primitive in &assets.primitives {
                command_buffer_builder = command_buffer_builder
                    .draw(
                        graphics.draw1_eraser_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        graphics.primitives_vertex_buffers[primitive.0].clone(),
                        (view_set.clone(), dynamic_draw_set.clone()),
                        (),
                    )
                    .unwrap();
            }
        }

        command_buffer_builder = command_buffer_builder
            .end_render_pass()
            .unwrap()
            .fill_buffer(graphics.tmp_erased_buffer.clone(), 0u32)
            .unwrap()
            .dispatch([graphics.dim[0]/64, graphics.dim[1]/64, 1], graphics.eraser1_pipeline.clone(), (graphics.eraser1_descriptor_set_0.clone(), graphics.eraser1_descriptor_set_1.clone()), ())
            .unwrap()
            // TODO: make velocity it configurable
            .dispatch([(::graphics::GROUP_COUNTER_SIZE/64) as u32, 1, 1], graphics.eraser2_pipeline.clone(), graphics.eraser2_descriptor_set.clone(), 6.0*config.dt())
            .unwrap();

        rendering.command_buffer = Some(command_buffer_builder.build().unwrap());

        // Compute second command
        let mut second_command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(graphics.device.clone(), graphics.queue.family()).unwrap()
            .begin_render_pass(graphics.second_framebuffers[rendering.image_num.take().unwrap()].clone(), false, vec!())
            .unwrap()
            .draw(
                graphics.draw2_pipeline.clone(),
                screen_dynamic_state.clone(),
                graphics.fullscreen_vertex_buffer.clone(),
                (graphics.draw2_descriptor_set_0.clone(), graphics.draw2_descriptor_set_1.clone()),
                ()
            )
            .unwrap()
            .draw(
                graphics.cursor_pipeline.clone(),
                DynamicState {
                    viewports: Some(vec![Viewport {
                    origin: [
                        (graphics.dim[0] - graphics.cursor_tex_dim[0] * 2) as f32 / 2.0,
                        (graphics.dim[1] - graphics.cursor_tex_dim[1] * 2) as f32 / 2.0,
                    ],
                    depth_range: 0.0..1.0,
                    dimensions: [
                        (graphics.cursor_tex_dim[0] * 2) as f32,
                        (graphics.cursor_tex_dim[1] * 2) as f32,
                    ],
                    }]),
                    ..DynamicState::none()
                },
                graphics.cursor_vertex_buffer.clone(),
                graphics.cursor_descriptor_set.clone(),
                ()
            )
            .unwrap();

        // Draw debug arrows
        if false {
            for arrow in ::graphics::DEBUG_ARROWS.draw() {
                let world_trans_subbuffer = graphics
                    .debug_arrow_world_uniform_buffer
                    .next(arrow.1)
                    .unwrap();

                // This is not optimised.
                let debug_arrow_set = Arc::new(PersistentDescriptorSet::start(graphics.debug_pipeline.clone(), 0)
                    .add_buffer(world_trans_subbuffer).unwrap()
                    .build().unwrap()
                );

                second_command_buffer_builder = second_command_buffer_builder
                    .draw(
                        graphics.debug_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        graphics.debug_arrow_vertex_buffer.clone(),
                        (view_set.clone(), debug_arrow_set.clone()),
                        arrow.0
                    )
                    .unwrap();
            }
        }

        // Build imgui
        let ui = imgui.frame(
            rendering.size_points.take().unwrap(),
            rendering.size_pixels.take().unwrap(),
            config.dt().clone(),
        );
        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ::imgui::ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.separator();
                ui.text(im_str!("This...is...imgui-rs!"));
            });

        // TODO: change imgui so that it use an iterator instead of a callback
        let ref_cell_cmd_builder = RefCell::new(Some(second_command_buffer_builder));
        ui.render::<_, ()>(|ui, drawlist| {
            let mut cmd_builder = ref_cell_cmd_builder.borrow_mut().take().unwrap();
            // TODO: impl vertex for imgui in imgui
            let (vertex_buffer, vertex_buf_future) = ImmutableBuffer::from_iter(
                drawlist.vtx_buffer.iter().map(|vtx| {
                    ::graphics::SecondVertexImgui::from(vtx.clone())
                }),
                BufferUsage::vertex_buffer(),
                graphics.queue.clone(),
            ).unwrap();
            future.push(vertex_buf_future);

            let (index_buffer, index_buf_future) = ImmutableBuffer::from_iter(
                drawlist.idx_buffer.iter().cloned(),
                BufferUsage::index_buffer(),
                graphics.queue.clone(),
            ).unwrap();
            future.push(index_buf_future);

            let (width, height) = ui.imgui().display_size();
            // let (scale_width, scale_height) = ui.imgui().display_framebuffer_scale();

            let matrix = [
                [2.0 / width as f32, 0.0, 0.0, 0.0],
                [0.0, 2.0 / -(height as f32), 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [-1.0, 1.0, 0.0, 1.0],
            ];

            let (matrix, matrix_future) = ImmutableBuffer::from_data(
                matrix,
                BufferUsage::uniform_buffer(),
                graphics.queue.clone(),
            ).unwrap();
            future.push(matrix_future);

            let matrix_set = Arc::new(
                graphics
                    .imgui_matrix_descriptor_set_pool
                    .next()
                    .add_buffer(matrix)
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            for _cmd in drawlist.cmd_buffer {
                // TODO: dynamic scissor
                // Scissor {
                //     origin: [
                //         (cmd.clip_rect.x * scale_width) as i32,
                //         ((height - cmd.clip_rect.w) * scale_height) as i32,
                //     ],
                //     dimensions: [
                //         ((cmd.clip_rect.z - cmd.clip_rect.x) * scale_width) as u32,
                //         ((cmd.clip_rect.w - cmd.clip_rect.y) * scale_height) as u32,
                //     ],
                // }

                cmd_builder = cmd_builder
                    .draw_indexed(
                        graphics.imgui_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        vertex_buffer.clone(), index_buffer.clone(),
                        (matrix_set.clone(), graphics.imgui_descriptor_set.clone()),
                        ()
                    )
                    .unwrap();
            }
            *ref_cell_cmd_builder.borrow_mut() = Some(cmd_builder);
            Ok(())
        }).unwrap();

        let second_command_buffer_builder = ref_cell_cmd_builder.borrow_mut().take().unwrap();

        rendering.second_command_buffer = Some(
            second_command_buffer_builder
                .end_render_pass()
                .unwrap()
                .build()
                .unwrap(),
        );
    }
}

pub struct UpdateDynamicDrawEraserSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawEraserSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>);

    fn run(&mut self, (bodies, mut dynamic_graphics_assets, physic_world): Self::SystemData) {
        for (assets, body) in (&mut dynamic_graphics_assets, &bodies).join() {
            let trans = body.get(&physic_world).position() * assets.primitive_trans;
            assets.world_trans =
                ::graphics::shader::draw1_vs::ty::World { world: trans.unwrap().into() }
        }
    }
}

pub struct LifeSystem;

impl<'a> ::specs::System<'a> for LifeSystem {
    type SystemData = (::specs::WriteStorage<'a, ::component::DynamicDraw>,
     ::specs::WriteStorage<'a, ::component::DynamicEraser>,
     ::specs::WriteStorage<'a, ::component::Life>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (mut dynamic_draws, mut dynamic_erasers, mut lives, entities): Self::SystemData,
    ) {
        for (life, entity) in (&mut lives, &*entities).join() {
            if !life.0 {
                if dynamic_draws.get(entity).is_some() {
                    entities.delete(entity).unwrap();
                } else {
                    life.0 = true;
                    dynamic_draws.insert(entity, ::component::DynamicDraw);
                    dynamic_erasers.remove(entity).unwrap();
                }
            }
        }
    }
}

pub struct ShootSystem {
    collided: Vec<(::specs::Entity, f32)>,
}

impl ShootSystem {
    pub fn new() -> Self {
        ShootSystem { collided: vec![] }
    }
}

impl<'a> ::specs::System<'a> for ShootSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::WriteStorage<'a, ::component::Shooter>,
     ::specs::WriteStorage<'a, ::component::Life>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (bodies, aims, mut shooters, mut lifes, physic_world, config, entities): Self::SystemData,
    ) {
        for (aim, body, shooter, entity) in (&aims, &bodies, &mut shooters, &*entities).join() {
            let body_pos = body.get(&physic_world).position().clone();
            shooter.reload(config.dt().clone());

            let ray = ::ncollide::query::Ray {
                origin: ::na::Point3::from_coordinates(body_pos.translation.vector),
                dir: aim.dir,
            };

            let mut group = ::ncollide::world::CollisionGroups::new();
            // TODO: resolve hack with membership nphysic #82
            group.set_membership(&[::entity::LASER_GROUP]);
            group.set_whitelist(&[::entity::LASER_GROUP]);

            if shooter.do_shoot() {
                // TODO: factorise this.
                self.collided.clear();
                for (other_body, collision) in
                    physic_world.collision_world().interferences_with_ray(
                        &ray,
                        &group,
                    )
                {
                    if let ::nphysics::object::WorldObject::RigidBody(other_body) =
                        other_body.data
                    {
                        let other_entity =
                            ::component::PhysicBody::entity(physic_world.rigid_body(other_body));
                        if entity != other_entity {
                            self.collided.push((other_entity, collision.toi));
                        }
                    }
                }
                self.collided.sort_by(
                    |a, b| (a.1).partial_cmp(&b.1).unwrap(),
                );
                for collided in &self.collided {
                    if let Some(ref mut life) = lifes.get_mut(collided.0) {
                        life.0 = false;
                    }
                    break;
                }
            }
        }
    }
}

pub struct MazeMasterSystem;

impl<'a> ::specs::System<'a> for MazeMasterSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::WriteStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::WriteStorage<'a, ::component::Avoider>,
     ::specs::WriteStorage<'a, ::component::Bouncer>,
     ::specs::WriteStorage<'a, ::component::DynamicDraw>,
     ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::WriteStorage<'a, ::component::Life>,
     ::specs::WriteStorage<'a, ::component::Contactor>,
     ::specs::Fetch<'a, ::resource::Maze>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (players, mut bodies, mut momentums, mut avoiders, mut bouncers, mut dynamic_draws, mut dynamic_graphics_assets, mut lives, mut contactors, maze, mut physic_world, entities): Self::SystemData,
    ) {
        let avoider_population = 10;
        let bouncer_population = 10;
        let kill_distance = 15.0;
        let spawn_distance = 10;

        let player_pos = {
            let p = (&players, &bodies).join().last().unwrap().1.get(&physic_world).position_center();
            ::na::Vector3::new(p[0], p[1], p[2])
        };

        // kill too far entities
        {
            let kill_too_far = |body: &::component::PhysicBody, life: &mut ::component::Life| {
                let pos = body.get(&physic_world).position().translation.vector;
                if (pos - player_pos).norm() > kill_distance {
                    life.0 = false;
                }
            };

            for (_, body, life) in (&bouncers, &bodies, &mut lives).join() {
                kill_too_far(body, life);
            }
            for (_, body, life) in (&avoiders, &bodies, &mut lives).join() {
                kill_too_far(body, life);
            }
        }

        let avoider_len = avoiders.join().fold(0, |acc, _| acc + 1);
        let bouncer_len = bouncers.join().fold(0, |acc, _| acc + 1);

        if avoider_len == avoider_population && bouncer_len == bouncer_population {
            return;
        }

        let square = maze.free_in_square([player_pos[0] as usize, player_pos[1] as usize], spawn_distance);
        if square.is_empty() {
            panic!("maze is too small to be able to create entities");
        }
        let square_range = Range::new(0, square.len());
        let mut rng = ::rand::thread_rng();

        for _ in avoider_len..avoider_population {
            let pos = square[square_range.ind_sample(&mut rng)];
            let pos = [pos[0] as f32 + 0.5, pos[1] as f32 + 0.5];

            ::entity::create_avoider(pos, &mut momentums, &mut avoiders, &mut bodies, &mut dynamic_draws, &mut dynamic_graphics_assets, &mut lives, &mut physic_world, &entities);
        }

        for _ in bouncer_len..bouncer_population {
            let pos = square[square_range.ind_sample(&mut rng)];
            let pos = [pos[0] as f32 + 0.5, pos[1] as f32 + 0.5];

            ::entity::create_bouncer(pos, &mut momentums, &mut bouncers, &mut bodies, &mut dynamic_draws, &mut dynamic_graphics_assets, &mut lives, &mut contactors, &mut physic_world, &entities);
        }
    }
}
