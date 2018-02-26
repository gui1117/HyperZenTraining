use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{BufferUsage, ImmutableBuffer};
use vulkano::pipeline::viewport::Viewport;
use specs::Join;
use alga::general::SubsetOf;
use util::{high_byte, low_byte};
use std::sync::Arc;
use std::cell::RefCell;

pub struct DrawSystem;

impl<'a> ::specs::System<'a> for DrawSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::StaticDraw>,
        ::specs::ReadStorage<'a, ::component::DynamicDraw>,
        ::specs::ReadStorage<'a, ::component::DynamicEraser>,
        ::specs::ReadStorage<'a, ::component::DynamicHud>,
        ::specs::ReadStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::FetchMut<'a, ::resource::ErasedStatus>,
        ::specs::FetchMut<'a, ::resource::Rendering>,
        ::specs::FetchMut<'a, ::resource::ImGuiOption>,
        ::specs::FetchMut<'a, ::resource::Graphics>,
        ::specs::FetchMut<'a, ::resource::MenuState>,
        ::specs::Fetch<'a, ::resource::FpsCounter>,
        ::specs::Fetch<'a, ::resource::Save>,
        ::specs::Fetch<'a, ::resource::VulkanInstance>,
        ::specs::Fetch<'a, ::resource::UpdateTime>,
        ::specs::Fetch<'a, ::resource::DepthCoef>,
        ::specs::Fetch<'a, ::resource::Benchmarks>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
    );

    fn run(
        &mut self,
        (
            static_draws,
            dynamic_draws,
            dynamic_erasers,
            dynamic_huds,
            dynamic_graphics_assets,
            bodies,
            players,
            aims,
            mut erased_status,
            mut rendering,
            mut imgui,
            mut graphics,
            mut menu_state,
            fps_counter,
            save,
            vulkan_instance,
            update_time,
            depth_coef,
            benchmarks,
            physic_world,
        ): Self::SystemData,
    ) {
        let mut future = Vec::new();

        // Compute view uniform
        let (view_uniform_buffer_subbuffer, hud_view_uniform_buffer_subbuffer) = {
            let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();

            let player_pos = player_body.get(&physic_world).position().clone();
            let player_aim_dir = player_aim.rotation * ::na::Vector3::x();
            // IDEA: we can do some fun things by changing this value
            let camera_top = player_aim.rotation * ::na::Vector3::z();

            let view_matrix = {
                let i: ::na::Transform3<f32> = ::na::Similarity3::look_at_rh(
                    &::na::Point3::from_coordinates(::na::Vector3::from(
                        player_pos.translation.vector,
                    )),
                    &::na::Point3::from_coordinates(
                        ::na::Vector3::from(player_pos.translation.vector) + player_aim_dir,
                    ),
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
                // IDEA: change to 0.0001 it's funny
                0.05 * depth_coef.0,
                100.0,
            ).unwrap();

            let view_uniform = ::graphics::shader::draw1_vs::ty::View {
                view: view_matrix.into(),
                proj: proj_matrix.into(),
            };

            let hud_view_matrix = {
                let i: ::na::Transform3<f32> = ::na::Similarity3::look_at_rh(
                    &::na::Point3::new(0.0, 0.0, 0.0),
                    &::na::Point3::new(1.0, 0.0, 0.0),
                    &[0.0, 0.0, 1.0].into(),
                    1.0,
                ).to_superset();
                i.unwrap()
            };

            let hud_proj_matrix = ::na::Perspective3::new(
                graphics.dim[0] as f32 / graphics.dim[1] as f32,
                ::std::f32::consts::FRAC_PI_3,
                // IDEA: change to 0.0001 it's funny
                0.001 * depth_coef.0,
                0.3,
            ).unwrap();

            let hud_view_uniform = ::graphics::shader::draw1_vs::ty::View {
                view: hud_view_matrix.into(),
                proj: hud_proj_matrix.into(),
            };

            (
                graphics.view_uniform_buffer.next(view_uniform).unwrap(),
                graphics.view_uniform_buffer.next(hud_view_uniform).unwrap(),
            )
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

        // Compute view set
        let hud_view_set = Arc::new(
            graphics
                .draw1_view_descriptor_set_pool
                .next()
                .add_buffer(hud_view_uniform_buffer_subbuffer)
                .unwrap()
                .build()
                .unwrap(),
        );

        let screen_dynamic_state = DynamicState {
            viewports: Some(vec![
                Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [graphics.dim[0] as f32, graphics.dim[1] as f32],
                    depth_range: 0.0..1.0,
                },
            ]),
            ..DynamicState::none()
        };

        // Compute command
        let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
            graphics.device.clone(),
            graphics.queue.family(),
        ).unwrap()
            .begin_render_pass(
                graphics.framebuffer.clone(),
                false,
                vec![0u32.into(), 0u32.into(), 1f32.into(), 1f32.into()],
            )
            .unwrap();

        // Draw static
        for static_draw in static_draws.join() {
            debug_assert_eq!(
                graphics.primitives_vertex_buffers[static_draw.primitive].len(),
                static_draw.groups.len()
            );
            for i in 0..graphics.primitives_vertex_buffers[static_draw.primitive].len() {
                command_buffer_builder = command_buffer_builder
                    .draw(
                        graphics.draw1_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        graphics.primitives_vertex_buffers[static_draw.primitive][i].clone(),
                        (view_set.clone(), static_draw.set.clone()),
                        ::graphics::shader::draw1_fs::ty::Group {
                            group_hb: high_byte(static_draw.groups[i] as u32),
                            group_lb: low_byte(static_draw.groups[i] as u32),
                            color: static_draw.color as u32,
                        },
                    )
                    .unwrap();
            }
        }

        // TODO: factorise draw loops

        // Draw dynamic
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

            debug_assert_eq!(
                graphics.primitives_vertex_buffers[assets.primitive].len(),
                assets.groups.len()
            );
            for i in 0..graphics.primitives_vertex_buffers[assets.primitive].len() {
                command_buffer_builder = command_buffer_builder
                    .draw(
                        graphics.draw1_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        graphics.primitives_vertex_buffers[assets.primitive][i].clone(),
                        (view_set.clone(), dynamic_draw_set.clone()),
                        ::graphics::shader::draw1_fs::ty::Group {
                            group_hb: high_byte(assets.groups[i] as u32),
                            group_lb: low_byte(assets.groups[i] as u32),
                            color: assets.color as u32,
                        },
                    )
                    .unwrap();
            }
        }

        command_buffer_builder = command_buffer_builder.next_subpass(false).unwrap();

        // Draw eraser
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

            for vertex_buffer in &graphics.primitives_vertex_buffers[assets.primitive] {
                command_buffer_builder = command_buffer_builder
                    .draw(
                        graphics.draw1_eraser_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        vertex_buffer.clone(),
                        (view_set.clone(), dynamic_draw_set.clone()),
                        (),
                    )
                    .unwrap();
            }
        }

        command_buffer_builder = command_buffer_builder.next_subpass(false).unwrap();

        // Draw hud
        for (_, assets) in (&dynamic_huds, &dynamic_graphics_assets).join() {
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

            debug_assert_eq!(
                graphics.primitives_vertex_buffers[assets.primitive].len(),
                assets.groups.len()
            );
            for i in 0..graphics.primitives_vertex_buffers[assets.primitive].len() {
                command_buffer_builder = command_buffer_builder
                    .draw(
                        graphics.draw1_hud_pipeline.clone(),
                        screen_dynamic_state.clone(),
                        graphics.primitives_vertex_buffers[assets.primitive][i].clone(),
                        (hud_view_set.clone(), dynamic_draw_set.clone()),
                        ::graphics::shader::draw1_fs::ty::Group {
                            group_hb: high_byte(assets.groups[i] as u32),
                            group_lb: low_byte(assets.groups[i] as u32),
                            color: assets.color as u32,
                        },
                    )
                    .unwrap();
            }
        }

        command_buffer_builder = command_buffer_builder
            .end_render_pass()
            .unwrap();

        if erased_status.need_buffer_clear {
            erased_status.need_buffer_clear = false;
            command_buffer_builder = command_buffer_builder
                .fill_buffer(graphics.tmp_erased_buffer.clone(), 0u32)
                .unwrap()
        }

        assert!(::graphics::GROUP_COUNTER_SIZE % 64 == 0);
        let x_iteration = graphics.dim[0] / 64 + if graphics.dim[0] % 64 != 0 { 1 } else { 0 };
        let y_iteration = graphics.dim[1] / 64 + if graphics.dim[1] % 64 != 0 { 1 } else { 0 };
        command_buffer_builder = command_buffer_builder
            .fill_buffer(graphics.tmp_erased_buffer.clone(), 0u32)
            .unwrap()
            .dispatch(
                [x_iteration, y_iteration, 1],
                graphics.eraser1_pipeline.clone(),
                (
                    graphics.eraser1_descriptor_set_0.clone(),
                    graphics.eraser1_descriptor_set_1.clone(),
                ),
                (),
            )
            .unwrap()
            .dispatch(
                [(::graphics::GROUP_COUNTER_SIZE / 64) as u32, 1, 1],
                graphics.eraser2_pipeline.clone(),
                graphics.eraser2_descriptor_set.clone(),
                update_time.0 / ::CONFIG.eraser_time,
            )
            .unwrap();

        rendering.command_buffer = Some(command_buffer_builder.build().unwrap());

        // Compute second command
        let second_command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
            graphics.device.clone(),
            graphics.queue.family(),
        ).unwrap()
            .begin_render_pass(
                graphics.second_framebuffers[rendering.image_num.take().unwrap()].clone(),
                false,
                vec![],
            )
            .unwrap()
            .draw(
                graphics.draw2_pipeline.clone(),
                screen_dynamic_state.clone(),
                graphics.fullscreen_vertex_buffer.clone(),
                (
                    graphics.draw2_descriptor_set_0.clone(),
                    graphics.draw2_descriptor_set_1.clone(),
                ),
                (),
            )
            .unwrap()
            .draw(
                graphics.cursor_pipeline.clone(),
                DynamicState {
                    viewports: Some(vec![
                        Viewport {
                            origin: [
                                (graphics.dim[0] - graphics.cursor_tex_dim[0] * 2) as f32 / 2.0,
                                (graphics.dim[1] - graphics.cursor_tex_dim[1] * 2) as f32 / 2.0,
                            ],
                            depth_range: 0.0..1.0,
                            dimensions: [
                                (graphics.cursor_tex_dim[0] * 2) as f32,
                                (graphics.cursor_tex_dim[1] * 2) as f32,
                            ],
                        },
                    ]),
                    ..DynamicState::none()
                },
                graphics.cursor_vertex_buffer.clone(),
                graphics.cursor_descriptor_set.clone(),
                (),
            )
            .unwrap();

        // Build imgui
        let ui = imgui.as_mut().unwrap().frame(
            rendering.size_points.take().unwrap(),
            rendering.size_pixels.take().unwrap(),
            ::CONFIG.dt(),
        );
        menu_state.build_ui(&ui, &save, &vulkan_instance);
        if false {
            ui.window(im_str!("Debug"))
                .size((100.0, 100.0), ::imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    ui.text(format!("fps: {}", fps_counter.0));
                    ui.separator();
                    for benchmark in &*benchmarks {
                        ui.text(format!("{}", benchmark));
                    }
                });
        }

        // TODO: change imgui so that it use an iterator instead of a callback
        let ref_cell_cmd_builder = RefCell::new(Some(second_command_buffer_builder));
        ui.render::<_, ()>(|ui, drawlist| {
            let mut cmd_builder = ref_cell_cmd_builder.borrow_mut().take().unwrap();
            // TODO: impl vertex for imgui in imgui
            let (vertex_buffer, vertex_buf_future) = ImmutableBuffer::from_iter(
                drawlist
                    .vtx_buffer
                    .iter()
                    .map(|vtx| ::graphics::SecondVertexImgui::from(vtx.clone())),
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
                        vertex_buffer.clone(),
                        index_buffer.clone(),
                        (matrix_set.clone(), graphics.imgui_descriptor_set.clone()),
                        (),
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
