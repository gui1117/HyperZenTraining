use specs::Join;
use alga::general::SubsetOf;
use std::sync::Arc;
use std::collections::BTreeMap;

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
                        let mut move_vector = ::na::zero();
                        if self.directions.is_empty() {
                            momentum.force_direction = move_vector;
                        } else {
                            for &direction in &self.directions {
                                match direction {
                                    ::util::Direction::Forward => move_vector[0] = 1.0,
                                    ::util::Direction::Backward => move_vector[0] = -1.0,
                                    ::util::Direction::Left => move_vector[1] = 1.0,
                                    ::util::Direction::Right => move_vector[1] = -1.0,
                                }
                            }
                            move_vector = ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, - control.pointer[0])) * move_vector;
                            momentum.force_direction = move_vector.normalize();
                        }
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

        let mut resolutions: BTreeMap<_, Vec<::na::Vector3<f32>>> = BTreeMap::new();
        println!("################################");
        for (co1, co2, contact) in col_world.contacts() {
            println!("#############");
            println!("{:#?}", contact);
            if momentums.get(co1.data).is_some() {
                //TODO USE FORCE CONSTRAINT!
                //OR USE THE MORE DEPTH FOR EACH OTHER WALL
                let normal = contact.normal;
                let depth = -contact.depth;

                let resolution = resolutions.entry(co1.uid).or_insert(vec!());
                resolution.push(depth*normal);

//                 if (depth*normal.x).abs() > resolution.0.abs() {
//                     resolution.0 = depth*normal.x;
//                 }
//                 if (depth*normal.y).abs() > resolution.1.abs() {
//                     resolution.1 = depth*normal.y;
//                 }
                // let resolution = match resolutions.entry(co1.uid) {
                //     Entry::Vacant(_) => depth*normal,
                //     Entry::Occupied(entry) => {
                //         let old_vector = *entry.get();
                //         let (larger, smaller) = if old_vector.norm() > depth.abs() {
                //             (old_vector, depth*normal)
                //         } else {
                //             (depth*normal, old_vector)
                //         };
                //         larger + smaller - smaller.dot(&larger)*larger.normalize()
                //     },
                // };
                // resolutions.insert(co1.uid, resolution);
                println!("{:#?}", resolution);
            }
            if momentums.get(co2.data).is_some() {
                unimplemented!();
            }
        }

        for (uid, res) in resolutions {
            let pos = col_world.collision_object(uid).unwrap().position;
            let mut a = ::na::Vector3::new(0.0, 0.0, 0.0);
            for x in &res {
                a += x;
            }
            a *= 1.0/res.len() as f32;
            col_world.deferred_set_position(uid, ::na::Translation3::from_vector(a)*pos);
        }
        col_world.perform_position_update();
    }
}

pub struct DrawSystem;

impl<'a> ::specs::System<'a> for DrawSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::StaticDraw>,
        ::specs::ReadStorage<'a, ::component::DynamicDraw>,
        ::specs::ReadStorage<'a, ::component::ColBody>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::FetchMut<'a, ::resource::Rendering>,
        ::specs::Fetch<'a, ::resource::ColWorld>,
        ::specs::Fetch<'a, ::resource::Control>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (static_draws, dynamic_draws, col_bodies, players, mut rendering, col_world, control, graphics, entities): Self::SystemData) {
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
                    graphics.cuboid_vertex_buffer.clone(),
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
                    graphics.cuboid_vertex_buffer.clone(),
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

pub struct UpdateDynamicDrawSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::ColBody>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::Fetch<'a, ::resource::ColWorld>,
        ::specs::Entities<'a>,
    );

    fn run(&mut self, (col_bodies, mut dynamic_draws, col_world, entities): Self::SystemData) {
        for (dynamic_draw, _, entity) in (&mut dynamic_draws, &col_bodies, &*entities).join() {
            let pos = col_world.collision_object(entity.id() as usize).unwrap().position;

            // TODO second arg !
            let trans: ::na::Transform3<f32> = ::na::Similarity3::from_isometry(pos, 0.1)
                .to_superset();
            dynamic_draw.world_trans = ::graphics::shader::vs::ty::World { world: trans.unwrap().into() }
        }
    }
}
