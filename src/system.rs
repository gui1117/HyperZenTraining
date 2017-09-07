use specs::Join;
use alga::general::SubsetOf;
use std::sync::Arc;

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
