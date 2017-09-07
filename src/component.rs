use std::sync::Arc;

#[derive(Default)]
pub struct ColBody;

impl ::specs::Component for ColBody {
    type Storage = ::specs::NullStorage<Self>;
}

impl ColBody {
    pub fn add(world: &mut ::specs::World, entity: ::specs::Entity, position: ::ColPosition, shape: ::ColShape, group: ::ColGroup) {
        let mut col_world = world.write_resource::<::ColWorld>();
        col_world.deferred_add(entity.id() as usize, position, shape, group, ::ncollide::world::GeometricQueryType::Contacts(0.0), ());

        world.write::<ColBody>().insert(entity, ColBody);
    }
}

pub struct StaticDraw {
    buffer: Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<[::graphics::Vertex]>>,
    set: Arc<::vulkano::descriptor::descriptor_set::PersistentDescriptorSet<Arc<::vulkano::pipeline::GraphicsPipeline<::vulkano::pipeline::vertex::SingleBufferDefinition<::graphics::Vertex>, Box<::vulkano::descriptor::PipelineLayoutAbstract + Sync + Send>, Arc<::vulkano::framebuffer::RenderPass<::graphics::render_pass::CustomRenderPassDesc>>>>, ((), ::vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf<Arc<::vulkano::buffer::CpuAccessibleBuffer<::graphics::shader::vs::ty::World>>>)>>,
}
