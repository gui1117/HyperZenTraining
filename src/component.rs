use std::sync::Arc;

// TODO check storage ???
#[derive(Default)]
pub struct Player;

impl ::specs::Component for Player {
    type Storage = ::specs::NullStorage<Self>;
}

#[derive(Default)]
pub struct ColBody;

impl ::specs::Component for ColBody {
    type Storage = ::specs::NullStorage<Self>;
}

impl ColBody {
    pub fn add(world: &mut ::specs::World, entity: ::specs::Entity, position: ::ColPosition, shape: ::ColShape, group: ::ColGroup) {
        let mut col_world = world.write_resource::<::ColWorld>();
        col_world.deferred_add(entity.id() as usize, position, shape, group, ::ncollide::world::GeometricQueryType::Contacts(0.0), ());

        match world.write::<ColBody>().insert(entity, ColBody) {
            ::specs::InsertResult::Inserted => (),
            _ => panic!("cannot insert colbody to entity"),
        };
    }
}

pub struct StaticDraw {
    pub constant: u32,
    pub uniform_buffer: Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<::graphics::shader::vs::ty::World>>,
    pub set: Arc<::vulkano::descriptor::descriptor_set::PersistentDescriptorSet<Arc<::vulkano::pipeline::GraphicsPipeline<::vulkano::pipeline::vertex::SingleBufferDefinition<::graphics::Vertex>, Box<::vulkano::descriptor::PipelineLayoutAbstract + Sync + Send>, Arc<::vulkano::framebuffer::RenderPass<::graphics::render_pass::CustomRenderPassDesc>>>>, ((), ::vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf<Arc<::vulkano::buffer::CpuAccessibleBuffer<::graphics::shader::vs::ty::World>>>)>>,
}

impl ::specs::Component for StaticDraw {
    type Storage = ::specs::VecStorage<Self>;
}

impl StaticDraw {
    pub fn add(world: &mut ::specs::World, entity: ::specs::Entity, group: u32, world_trans: ::graphics::shader::vs::ty::World) {
        let graphics = world.read_resource::<::resource::Graphics>();

        let uniform_buffer =
            ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::<::graphics::shader::vs::ty::World>::from_data(
                graphics.device.clone(),
                ::vulkano::buffer::BufferUsage::uniform_buffer(),
                world_trans,
                ).expect("failed to create buffer");

        let set = Arc::new(
            ::vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
                graphics.pipeline.clone(),
                0,
                ).add_buffer(uniform_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
            );

        let static_draw = StaticDraw {
            constant: group,
            uniform_buffer,
            set,
        };

        match world.write::<StaticDraw>().insert(entity, static_draw) {
            ::specs::InsertResult::Inserted => (),
            _ => panic!("cannot insert colbody to entity"),
        };
    }
}
