use specs::Join;
use alga::general::SubsetOf;
use std::sync::Arc;

// TODO: get mouse from axis and check if there are differences because of acceleration
pub struct PlayerControlSystem {
    directions: Vec<::util::Direction>,
    pointer: [f32; 2],
}

impl PlayerControlSystem {
    pub fn new() -> Self {
        PlayerControlSystem {
            directions: vec!(),
            pointer: [0.0, 0.0],
        }
    }
}

impl<'a> ::specs::System<'a> for PlayerControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::WriteStorage<'a, ::component::Aim>,
     ::specs::WriteStorage<'a, ::component::Shooter>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::WinitEvents>,
     ::specs::Fetch<'a, ::resource::Graphics>,
     ::specs::Fetch<'a, ::resource::Config>);

    fn run(
        &mut self,
        (players, mut aims, mut shooters, mut momentums, events, graphics, config): Self::SystemData,
    ) {
        let (_, player_aim, player_shooter, player_momentum) = (&players, &mut aims, &mut shooters, &mut momentums).join().next().unwrap();
        for ev in events.iter() {
            match *ev {
                ::winit::Event::WindowEvent {
                    event: ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state, .. }, ..
                } => {
                    match state {
                        ::winit::ElementState::Pressed => player_shooter.set_shoot(true),
                        ::winit::ElementState::Released => player_shooter.set_shoot(false),
                    }
                }
                ::winit::Event::WindowEvent {
                    event: ::winit::WindowEvent::MouseMoved { position: (dx, dy), .. }, ..
                } => {
                    self.pointer[0] += (dx as f32 - graphics.width as f32 / 2.0) /
                        config.mouse_sensibility;
                    self.pointer[1] += (dy as f32 - graphics.height as f32 / 2.0) /
                        config.mouse_sensibility;
                    self.pointer[1] = self.pointer[1]
                        .min(::std::f32::consts::FRAC_PI_2)
                        .max(-::std::f32::consts::FRAC_PI_2);
                }
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
                    ::util::Direction::Forward => move_vector[0] = 1.0,
                    ::util::Direction::Backward => move_vector[0] = -1.0,
                    ::util::Direction::Left => move_vector[1] = 1.0,
                    ::util::Direction::Right => move_vector[1] = -1.0,
                }
            }
            move_vector = (::na::Rotation3::new(
                ::na::Vector3::new(0.0, 0.0, -self.pointer[0]),
            ) * move_vector)
                .normalize();
            player_momentum.direction = move_vector;
        }
    }
}

pub struct AvoiderControlSystem;

impl<'a> ::specs::System<'a> for AvoiderControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::ReadStorage<'a, ::component::PhysicRigidBodyHandle>,
     ::specs::WriteStorage<'a, ::component::Avoider>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Maze>);

    fn run(
        &mut self,
        (players, aims, bodies, mut avoiders, mut momentums, physic_world, maze): Self::SystemData,
    ) {
        let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();

        let player_pos = player_body.get(&physic_world).body.position().clone();

        for (avoider, momentum, body) in (&mut avoiders, &mut momentums, &bodies).join() {
            let avoider_pos = body.get(&physic_world).body.position().clone();

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

                ((goal_pos - avoider_pos.translation.vector).normalize(), 1f32)
            };

            let (avoid_direction, avoid_coef) = {
                let avoider_pos_rel_player = avoider_pos.translation.vector - player_pos.translation.vector;
                let avoid_vector = avoider_pos_rel_player - avoider_pos_rel_player.dot(&player_aim.dir)*player_aim.dir;
                if avoid_vector.norm() != 0.0 {
                    let avoid_norm = avoid_vector.norm();
                    let avoid_direction = avoid_vector.normalize();
                    if avoid_norm > 0.5 {
                        (avoid_direction, 0f32)
                    } else {
                        // TODO: COEFFICENT ??
                        (avoid_direction, 1f32)//1.0/avoid_norm)
                    }
                } else {
                    let random = ::na::Vector3::new_random();
                    ((random - random.dot(&player_aim.dir)*player_aim.dir).normalize(), 1f32)//1000f32)
                }
            };

            momentum.direction = (goal_coef*goal_direction + avoid_coef*avoid_direction).normalize();
        }
    }
}

pub struct PhysicSystem;

impl<'a> ::specs::System<'a> for PhysicSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Momentum>,
     ::specs::WriteStorage<'a, ::component::PhysicRigidBodyHandle>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>);

    fn run(&mut self, (player, momentums, mut bodies, config, mut physic_world): Self::SystemData) {
        println!("BEGIN PHYSIC HERE");
        for (momentum, body) in (&momentums, &mut bodies).join() {
            println!("physic borrow BEGIN");
            let mut body = body.get_mut(&mut physic_world).body;
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
            println!("physic borrow END");

            // TODO: gravity if not touching floor
            // body.append_lin_force(10.0*::na::Vector3::new(0.0,0.0,-1.0));
        }
        for _ in 0..2 {
            physic_world.step(config.dt / 2.);
        }
        for (_, body) in (&player, &mut bodies).join() {
            let mut body = body.get_mut(&mut physic_world).body;
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
     ::specs::ReadStorage<'a, ::component::PhysicRigidBodyHandle>,
     ::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::FetchMut<'a, ::resource::Rendering>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Graphics>);

fn run(&mut self, (static_draws, dynamic_draws, bodies, players, aims, mut rendering, physic_world, graphics): Self::SystemData){
        // Compute view uniform
        let view_uniform_buffer_subbuffer = {
            let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();

            let player_pos = player_body.get(&physic_world).body.position().clone();

            // IDEA: if we change -player.x here to + then it is fun
            let camera_top = if player_aim.dir[2].abs() > 0.8 {
                ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -player_aim.x_dir)) * ::na::Vector3::x() * - player_aim.dir[2].signum()
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
                graphics.width as f32 / graphics.height as f32,
                ::std::f32::consts::FRAC_PI_3,
                0.01,
                100.0,
            ).unwrap();

            let view_uniform = ::graphics::shader::vs::ty::View {
                view: view_matrix.into(),
                proj: proj_matrix.into(),
            };

            graphics.view_uniform_buffer.next(view_uniform).unwrap()
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
        let mut command_buffer_builder = ::vulkano::command_buffer::AutoCommandBufferBuilder::new(
            graphics.device.clone(),
            graphics.queue.family(),
        ).unwrap()
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
                    ::graphics::shader::fs::ty::Group { group: static_draw.group },
                )
                .unwrap();
        }

        for dynamic_draw in dynamic_draws.join() {
            let world_trans_subbuffer = dynamic_draw.uniform_buffer_pool
                .next(dynamic_draw.world_trans)
                .unwrap();

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
                    graphics.pyramid_vertex_buffer.clone(),
                    (view_set.clone(), dynamic_draw_set),
                    ::graphics::shader::fs::ty::Group { group: dynamic_draw.group },
                )
                .unwrap();
        }

        rendering.command_buffer = Some(
            command_buffer_builder
                .end_render_pass()
                .unwrap()
                .build()
                .unwrap(),
        );

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

pub struct UpdateDynamicDrawSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::PhysicRigidBodyHandle>,
     ::specs::WriteStorage<'a, ::component::DynamicDraw>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>);

    fn run(&mut self, (bodies, mut dynamic_draws, physic_world): Self::SystemData) {
        for (dynamic_draw, body) in (&mut dynamic_draws, &bodies).join() {
            let trans = body.get(&physic_world).body.position() * dynamic_draw.primitive_trans;
            dynamic_draw.world_trans =
                ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
        }
    }
}

pub struct ShootSystem;

// TODO: not shoot yourself and shoot in one direction only
impl<'a> ::specs::System<'a> for ShootSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::PhysicRigidBodyHandle>,
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::WriteStorage<'a, ::component::Shooter>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::Config>);

    fn run(&mut self, (bodies, aims, mut shooters, mut lifes, physic_world, config): Self::SystemData) {
        for (aim, body, shooter) in (&aims, &bodies, &mut shooters).join() {
            let body_pos = body.get(&physic_world).body.position().clone();
            shooter.reload(config.dt);

            let ray = ::ncollide::query::Ray {
                origin: ::na::Point3::from_coordinates(body_pos.translation.vector),
                dir: aim.dir,
            };

            let group = ::ncollide::world::CollisionGroups::new();

            if shooter.do_shoot() {
                for (entity, _body, _collision) in physic_world.interferences_with_ray(&ray, &group) {
                    if let Some(ref mut life) = lifes.get_mut(entity) {
                        life.0 -= 1;
                    }
                    println!("borrow filter map here END");
                }
            }
        }
        println!("END SYSTEM SHOOT");
    }
}
