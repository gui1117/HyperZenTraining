use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::swapchain::{self, Swapchain};
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode,
                       UnnormalizedSamplerAddressMode};
use vulkano::image::{AttachmentImage, Dimensions, ImageUsage, ImmutableImage, SwapchainImage};
// TODO: replace CpuAccessible by something else ?
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool, DeviceLocalBuffer,
                      ImmutableBuffer};
use vulkano::framebuffer::{Framebuffer, RenderPass, RenderPassDesc, Subpass};
use vulkano::pipeline::{ComputePipeline, GraphicsPipeline};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::descriptor::pipeline_layout::PipelineLayout;
use vulkano::descriptor::descriptor_set::{FixedSizeDescriptorSetsPool, PersistentDescriptorSet,
                                          PersistentDescriptorSetBuf, PersistentDescriptorSetImg,
                                          PersistentDescriptorSetSampler};
use vulkano::instance::PhysicalDevice;
use vulkano::format;
use vulkano::sync::{now, GpuFuture};

use std::sync::Arc;
use std::fs::File;

pub mod shader;
pub mod render_pass;
mod primitives;
mod colors;
pub mod font;

pub use self::primitives::primitive::Primitive;
pub use self::colors::Color;
pub use self::primitives::primitive::GROUP_COUNTER_SIZE;

pub fn resizer(x: f32, y: f32, z: f32) -> ::na::Transform3<f32> {
    let mut resizer: ::na::Transform3<f32> = ::na::one();
    resizer[(0, 0)] = x;
    resizer[(1, 1)] = y;
    resizer[(2, 2)] = z;
    resizer
}

#[derive(Debug, Clone)]
pub struct Vertex {
    position: [f32; 3],
}
impl_vertex!(Vertex, position);

#[derive(Debug, Clone)]
pub struct SecondVertex {
    position: [f32; 2],
}
impl_vertex!(SecondVertex, position);

#[derive(Debug, Clone)]
pub struct SecondVertexImgui {
    pos: [f32; 2],
    uv: [f32; 2],
    col: [f32; 4],
}

impl_vertex!(SecondVertexImgui, pos, uv, col);
impl From<::imgui::ImDrawVert> for SecondVertexImgui {
    fn from(vertex: ::imgui::ImDrawVert) -> Self {
        let r = vertex.col as u8 as f32;
        let g = (vertex.col >> 8) as u8 as f32;
        let b = (vertex.col >> 16) as u8 as f32;
        let a = (vertex.col >> 24) as u8 as f32;
        SecondVertexImgui {
            pos: [vertex.pos.x, vertex.pos.y],
            uv: [vertex.uv.x, vertex.uv.y],
            col: [r, g, b, a],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugVertex {
    position: [f32; 3],
    normal: [f32; 3],
}
impl_vertex!(DebugVertex, position, normal);

#[derive(Clone)]
// FIXME: use abstract types instead
pub struct Data {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,

    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<SwapchainImage>>,
    pub dim: [u32; 2],
    pub cursor_tex_dim: [u32; 2],

    pub primitives_vertex_buffers: Vec<Vec<Arc<ImmutableBuffer<[Vertex]>>>>,
    pub debug_arrow_vertex_buffer: Arc<ImmutableBuffer<[DebugVertex]>>,
    pub fullscreen_vertex_buffer: Arc<ImmutableBuffer<[SecondVertex]>>,
    pub cursor_vertex_buffer: Arc<ImmutableBuffer<[SecondVertex]>>,

    pub view_uniform_buffer: CpuBufferPool<::graphics::shader::draw1_vs::ty::View>,
    pub world_uniform_static_buffer: CpuBufferPool<::graphics::shader::draw1_vs::ty::World>,
    pub world_uniform_buffer: CpuBufferPool<::graphics::shader::draw1_vs::ty::World>,
    pub debug_arrow_world_uniform_buffer: CpuBufferPool<::graphics::shader::debug_vs::ty::World>,
    pub tmp_erased_buffer: Arc<DeviceLocalBuffer<[u32; 65536]>>,
    pub erased_buffer: Arc<CpuAccessibleBuffer<[f32; 65536]>>,

    pub render_pass: Arc<RenderPass<render_pass::CustomRenderPassDesc>>,
    pub second_render_pass: Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>,

    pub framebuffer: Arc<Framebuffer<Arc<RenderPass<render_pass::CustomRenderPassDesc>>, (((((), Arc<AttachmentImage>), Arc<AttachmentImage>), Arc<AttachmentImage>), Arc<AttachmentImage>)>>,
    pub second_framebuffers: Vec<Arc<Framebuffer<Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>, ((), Arc<SwapchainImage>)>>>,

    pub draw1_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>,
    pub draw1_eraser_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>,
    pub draw1_hud_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>,
    pub eraser1_pipeline: Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser1_cs::Layout>>>,
    pub eraser2_pipeline: Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser2_cs::Layout>>>,
    pub draw2_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,
    pub cursor_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,
    pub imgui_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<SecondVertexImgui>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,
    pub debug_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<DebugVertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,

    pub draw1_view_descriptor_set_pool: FixedSizeDescriptorSetsPool<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>>,
    pub draw1_dynamic_descriptor_set_pool: FixedSizeDescriptorSetsPool<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>>,
    pub imgui_matrix_descriptor_set_pool: FixedSizeDescriptorSetsPool<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertexImgui>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>>,

    pub cursor_descriptor_set: Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<::graphics::render_pass::SecondCustomRenderPassDesc>>>>, (((), PersistentDescriptorSetImg<Arc<ImmutableImage<format::R8G8B8A8Unorm>>>), PersistentDescriptorSetSampler)>>,
    pub imgui_descriptor_set: Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertexImgui>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<::graphics::render_pass::SecondCustomRenderPassDesc>>>>, (((), PersistentDescriptorSetImg<Arc<ImmutableImage<format::R8G8B8A8Unorm>>>), PersistentDescriptorSetSampler)>>,

    pub eraser1_descriptor_set_0: Arc<PersistentDescriptorSet<Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser1_cs::Layout>>>, (((((), PersistentDescriptorSetImg<Arc<AttachmentImage>>), PersistentDescriptorSetSampler), PersistentDescriptorSetImg<Arc<AttachmentImage>>), PersistentDescriptorSetSampler)>>,
    pub eraser1_descriptor_set_1: Arc<PersistentDescriptorSet<Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser1_cs::Layout>>>, ((), PersistentDescriptorSetBuf<Arc<DeviceLocalBuffer<[u32; 65536]>>>)>>,
    pub eraser2_descriptor_set: Arc<PersistentDescriptorSet<Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser2_cs::Layout>>>, (((), PersistentDescriptorSetBuf<Arc<DeviceLocalBuffer<[u32; 65536]>>>), PersistentDescriptorSetBuf<Arc<CpuAccessibleBuffer<[f32; 65536]>>>)>>,
    pub draw2_descriptor_set_0: Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>, (((), PersistentDescriptorSetImg<Arc<AttachmentImage>>), PersistentDescriptorSetSampler)>>,
    pub draw2_descriptor_set_1: Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>, (((), PersistentDescriptorSetBuf<Arc<ImmutableBuffer<[[f32; 4]]>>>), PersistentDescriptorSetBuf<Arc<CpuAccessibleBuffer<[f32; 65536]>>>)>>,
}

pub struct Graphics<'a> {
    pub physical: PhysicalDevice<'a>,
    pub data: Data,
}

impl<'a> Graphics<'a> {
    pub fn framebuffers_and_descriptors(
        device: Arc<Device>,
        queue: Arc<Queue>,
        images: &Vec<Arc<SwapchainImage>>,
        render_pass: &Arc<RenderPass<render_pass::CustomRenderPassDesc>>,
        second_render_pass: &Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>,
        eraser1_pipeline: &Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser1_cs::Layout>>>,
        draw2_pipeline: &Arc<GraphicsPipeline<SingleBufferDefinition<SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,
        imgui_pipeline: &Arc<GraphicsPipeline<SingleBufferDefinition<SecondVertexImgui>, Box<PipelineLayoutAbstract + Sync + Send>, ::Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,
        imgui: &mut ::imgui::ImGui,
    ) -> (
        Arc<Framebuffer<Arc<RenderPass<render_pass::CustomRenderPassDesc>>, (((((), Arc<AttachmentImage>), Arc<AttachmentImage>), Arc<AttachmentImage>), Arc<AttachmentImage>)>>,
        Vec<Arc<Framebuffer<Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>, ((), Arc<SwapchainImage>)>>>,
        Arc<PersistentDescriptorSet<Arc<ComputePipeline<PipelineLayout<::graphics::shader::eraser1_cs::Layout>>>, (((((), PersistentDescriptorSetImg<Arc<AttachmentImage>>), PersistentDescriptorSetSampler), PersistentDescriptorSetImg<Arc<AttachmentImage>>), PersistentDescriptorSetSampler)>>,
        Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::SecondCustomRenderPassDesc>>>>, (((), PersistentDescriptorSetImg<Arc<AttachmentImage>>), PersistentDescriptorSetSampler)>>,
        Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<::graphics::SecondVertexImgui>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<::graphics::render_pass::SecondCustomRenderPassDesc>>>>, (((), PersistentDescriptorSetImg<Arc<ImmutableImage<format::R8G8B8A8Unorm>>>), PersistentDescriptorSetSampler)>>,
){
        let imgui_texture = imgui
            .prepare_texture(|handle| {
                ImmutableImage::from_iter(
                    handle.pixels.iter().cloned(),
                    Dimensions::Dim2d {
                        width: handle.width,
                        height: handle.height,
                    },
                    format::R8G8B8A8Unorm,
                    queue.clone(),
                )
            })
            .unwrap().0;

        let imgui_descriptor_set = {
            Arc::new(
                PersistentDescriptorSet::start(imgui_pipeline.clone(), 1)
                    .add_sampled_image(
                        imgui_texture,
                        Sampler::new(
                            device.clone(),
                            Filter::Nearest,
                            Filter::Linear,
                            MipmapMode::Linear,
                            SamplerAddressMode::MirroredRepeat,
                            SamplerAddressMode::MirroredRepeat,
                            SamplerAddressMode::MirroredRepeat,
                            0.0,
                            1.0,
                            0.0,
                            0.0,
                        ).unwrap(),
                    )
                    .unwrap()
                    .build()
                    .unwrap(),
            )
        };

        let depth_buffer_attachment = AttachmentImage::transient(
            device.clone(),
            images[0].dimensions(),
            format::Format::D16Unorm,
        ).unwrap();

        let hud_depth_buffer_attachment = AttachmentImage::transient(
            device.clone(),
            images[0].dimensions(),
            format::Format::D16Unorm,
        ).unwrap();

        let tmp_image_attachment = {
            let usage = ImageUsage {
                color_attachment: true,
                sampled: true,
                ..ImageUsage::none()
            };
            AttachmentImage::with_usage(
                device.clone(),
                images[0].dimensions(),
                format::Format::R8G8B8A8Uint,
                usage,
            ).unwrap()
        };

        let tmp_erase_image_attachment = {
            let usage = ImageUsage {
                color_attachment: true,
                sampled: true,
                ..ImageUsage::none()
            };
            AttachmentImage::with_usage(
                device.clone(),
                images[0].dimensions(),
                format::Format::R8Uint,
                usage,
            ).unwrap()
        };

        let framebuffer = Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(tmp_image_attachment.clone())
                .unwrap()
                .add(tmp_erase_image_attachment.clone())
                .unwrap()
                .add(depth_buffer_attachment.clone())
                .unwrap()
                .add(hud_depth_buffer_attachment.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let second_framebuffers = images
            .iter()
            .map(|image| {
                Arc::new(
                    Framebuffer::start(second_render_pass.clone())
                        .add(image.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let eraser1_descriptor_set_0 = Arc::new(
            PersistentDescriptorSet::start(eraser1_pipeline.clone(), 0)
                .add_sampled_image(
                    tmp_image_attachment.clone(),
                    Sampler::unnormalized(
                        device.clone(),
                        Filter::Nearest,
                        UnnormalizedSamplerAddressMode::ClampToEdge,
                        UnnormalizedSamplerAddressMode::ClampToEdge,
                    ).unwrap(),
                )
                .unwrap()
                .add_sampled_image(
                    tmp_erase_image_attachment.clone(),
                    Sampler::unnormalized(
                        device.clone(),
                        Filter::Nearest,
                        UnnormalizedSamplerAddressMode::ClampToEdge,
                        UnnormalizedSamplerAddressMode::ClampToEdge,
                    ).unwrap(),
                )
                .unwrap()
                .build()
                .unwrap(),
        );

        let draw2_descriptor_set_0 = Arc::new(
            PersistentDescriptorSet::start(draw2_pipeline.clone(), 0)
                .add_sampled_image(
                    tmp_image_attachment.clone(),
                    Sampler::unnormalized(
                        device.clone(),
                        Filter::Nearest,
                        UnnormalizedSamplerAddressMode::ClampToEdge,
                        UnnormalizedSamplerAddressMode::ClampToEdge,
                    ).unwrap(),
                )
                .unwrap()
                .build()
                .unwrap(),
        );

        (
            framebuffer,
            second_framebuffers,
            eraser1_descriptor_set_0,
            draw2_descriptor_set_0,
            imgui_descriptor_set,
        )
    }
    pub fn new(window: &'a ::vulkano_win::Window, imgui: &mut ::imgui::ImGui) -> Graphics<'a> {
        // TODO: read config and save device
        let physical = PhysicalDevice::enumerate(&window.surface().instance())
            .next()
            .expect("no device available");

        let queue_family = physical
            .queue_families()
            .find(|&q| {
                q.supports_graphics() && q.supports_compute()
                    && window.surface().is_supported(q).unwrap_or(false)
            })
            .expect("couldn't find a graphical queue family");

        let (device, mut queues) = {
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            };

            Device::new(
                physical,
                physical.supported_features(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned(),
            ).expect("failed to create device")
        };

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let caps = window
                .surface()
                .capabilities(physical)
                .expect("failed to get surface capabilities");

            let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
            let image_usage = ImageUsage {
                sampled: true,
                color_attachment: true,
                ..ImageUsage::none()
            };

            Swapchain::new(
                device.clone(),
                window.surface().clone(),
                caps.min_image_count,
                format::Format::B8G8R8A8Srgb,
                dimensions,
                1,
                image_usage,
                &queue,
                swapchain::SurfaceTransform::Identity,
                swapchain::CompositeAlpha::Opaque,
                swapchain::PresentMode::Fifo,
                true,
                None,
            ).expect("failed to create swapchain")
        };

        let dim = images[0].dimensions();

        let (primitives_vertex_buffers, primitives_future) =
            primitives::instance_primitives(queue.clone());

        let (fullscreen_vertex_buffer, fullscreen_vertex_buffer_future) =
            ImmutableBuffer::from_iter(
                [
                    [-1.0f32, -1.0],
                    [-1.0, 1.0],
                    [1.0, -1.0],
                    [1.0, 1.0],
                    [1.0, -1.0],
                    [-1.0, 1.0],
                ].iter()
                    .cloned()
                    .map(|position| SecondVertex { position }),
                BufferUsage::vertex_buffer(),
                queue.clone(),
            ).expect("failed to create buffer");

        let (cursor_vertex_buffer, cursor_vertex_buffer_future) =
            ImmutableBuffer::from_iter(
                [
                    [-0.5f32, -0.5],
                    [-0.5, 0.5],
                    [0.5, -0.5],
                    [0.5, 0.5],
                    [0.5, -0.5],
                    [-0.5, 0.5],
                ].iter()
                    .cloned()
                    .map(|position| SecondVertex { position }),
                BufferUsage::vertex_buffer(),
                queue.clone(),
            ).expect("failed to create buffer");

        let (cursor_texture, cursor_tex_future) = {
            let file = File::open("assets/cursor.png").unwrap();
            let (info, mut reader) = ::png::Decoder::new(file).read_info().unwrap();
            assert_eq!(info.color_type, ::png::ColorType::RGBA);
            let mut buf = vec![0; info.buffer_size()];
            reader.next_frame(&mut buf).unwrap();

            ImmutableImage::from_iter(
                buf.into_iter(),
                Dimensions::Dim2d {
                    width: info.width,
                    height: info.height,
                },
                format::R8G8B8A8Unorm,
                queue.clone(),
            ).unwrap()
        };

        let cursor_sampler = Sampler::new(
            device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::ClampToEdge,
            SamplerAddressMode::ClampToEdge,
            SamplerAddressMode::ClampToEdge,
            0.0,
            1.0,
            0.0,
            0.0,
        ).unwrap();

        let cursor_tex_dim = cursor_texture.dimensions().width_height();

        let draw1_vs =
            shader::draw1_vs::Shader::load(device.clone()).expect("failed to create shader module");
        let draw1_fs =
            shader::draw1_fs::Shader::load(device.clone()).expect("failed to create shader module");
        let draw1_eraser_fs = shader::draw1_eraser_fs::Shader::load(device.clone())
            .expect("failed to create shader module");

        let eraser1_cs = shader::eraser1_cs::Shader::load(device.clone())
            .expect("failed to create shader module");
        let eraser2_cs = shader::eraser2_cs::Shader::load(device.clone())
            .expect("failed to create shader module");

        let draw2_vs =
            shader::draw2_vs::Shader::load(device.clone()).expect("failed to create shader module");
        let draw2_fs =
            shader::draw2_fs::Shader::load(device.clone()).expect("failed to create shader module");

        let cursor_vs = shader::cursor_vs::Shader::load(device.clone())
            .expect("failed to create shader module");
        let cursor_fs = shader::cursor_fs::Shader::load(device.clone())
            .expect("failed to create shader module");

        let imgui_vs =
            shader::imgui_vs::Shader::load(device.clone()).expect("failed to create shader module");
        let imgui_fs =
            shader::imgui_fs::Shader::load(device.clone()).expect("failed to create shader module");

        let debug_vs =
            shader::debug_vs::Shader::load(device.clone()).expect("failed to create shader module");
        let debug_fs =
            shader::debug_fs::Shader::load(device.clone()).expect("failed to create shader module");

        let render_pass = Arc::new(
            render_pass::CustomRenderPassDesc
                .build_render_pass(device.clone())
                .unwrap(),
        );

        let second_render_pass = Arc::new(
            render_pass::SecondCustomRenderPassDesc
                .build_render_pass(device.clone())
                .unwrap(),
        );

        let draw1_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(draw1_vs.main_entry_point(), ())
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(draw1_fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .sample_shading_enabled(1.0)
                .cull_mode_back()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let draw1_eraser_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(draw1_vs.main_entry_point(), ())
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(draw1_eraser_fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .sample_shading_enabled(1.0)
                .cull_mode_back()
                .render_pass(Subpass::from(render_pass.clone(), 1).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let draw1_hud_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(draw1_vs.main_entry_point(), ())
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(draw1_fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .sample_shading_enabled(1.0)
                .cull_mode_back()
                .render_pass(Subpass::from(render_pass.clone(), 2).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let eraser1_pipeline = Arc::new(
            ComputePipeline::new(device.clone(), &eraser1_cs.main_entry_point(), &()).unwrap(),
        );
        let eraser2_pipeline = Arc::new(
            ComputePipeline::new(device.clone(), &eraser2_cs.main_entry_point(), &()).unwrap(),
        );

        let draw2_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<SecondVertex>()
                .vertex_shader(draw2_vs.main_entry_point(), ())
                .triangle_list()
                .cull_mode_back()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(draw2_fs.main_entry_point(), ())
                .render_pass(Subpass::from(second_render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let cursor_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<SecondVertex>()
                .vertex_shader(cursor_vs.main_entry_point(), ())
                .triangle_list()
                .cull_mode_back()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(cursor_fs.main_entry_point(), ())
                .blend_alpha_blending()
                .render_pass(Subpass::from(second_render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let imgui_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<SecondVertexImgui>()
                .vertex_shader(imgui_vs.main_entry_point(), ())
                .triangle_list()
                .cull_mode_front()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(imgui_fs.main_entry_point(), ())
                .blend_alpha_blending()
                .render_pass(Subpass::from(second_render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let debug_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<DebugVertex>()
                .vertex_shader(debug_vs.main_entry_point(), ())
                .triangle_list()
                .cull_mode_back()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(debug_fs.main_entry_point(), ())
                .blend_alpha_blending()
                .render_pass(Subpass::from(second_render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let (framebuffer, second_framebuffers, eraser1_descriptor_set_0, draw2_descriptor_set_0, imgui_descriptor_set) =
            Graphics::framebuffers_and_descriptors(
                device.clone(),
                queue.clone(),
                &images,
                &render_pass,
                &second_render_pass,
                &eraser1_pipeline,
                &draw2_pipeline,
                &imgui_pipeline,
                imgui,
            );

        let view_uniform_buffer = CpuBufferPool::<::graphics::shader::draw1_vs::ty::View>::new(
            device.clone(),
            BufferUsage::uniform_buffer(),
        );

        let world_uniform_static_buffer =
            CpuBufferPool::<::graphics::shader::draw1_vs::ty::World>::new(
                device.clone(),
                BufferUsage::uniform_buffer(),
            );

        let world_uniform_buffer = CpuBufferPool::<::graphics::shader::draw1_vs::ty::World>::new(
            device.clone(),
            BufferUsage::uniform_buffer(),
        );

        let debug_arrow_world_uniform_buffer = CpuBufferPool::<
            ::graphics::shader::debug_vs::ty::World,
        >::new(device.clone(), BufferUsage::uniform_buffer());

        let (colors_buffer, colors_buf_future) = {
            let colors = colors::colors();
            ImmutableBuffer::from_iter(
                colors.into_iter(),
                // TODO: not all buffer usage
                BufferUsage::all(),
                queue.clone(),
            ).unwrap()
        };

        // TODO: not all buffer usage
        let tmp_erased_buffer = DeviceLocalBuffer::<[u32; GROUP_COUNTER_SIZE]>::new(
            device.clone(),
            BufferUsage::all(),
            vec![queue.family()].into_iter(),
        ).unwrap();
        let erased_buffer = CpuAccessibleBuffer::from_data(
            device.clone(),
            BufferUsage::all(),
            [1f32; GROUP_COUNTER_SIZE],
        ).unwrap();

        let eraser1_descriptor_set_1 = Arc::new(
            PersistentDescriptorSet::start(eraser1_pipeline.clone(), 1)
                .add_buffer(tmp_erased_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let draw2_descriptor_set_1 = Arc::new(
            PersistentDescriptorSet::start(draw2_pipeline.clone(), 1)
                .add_buffer(colors_buffer.clone())
                .unwrap()
                .add_buffer(erased_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let cursor_descriptor_set = Arc::new(
            PersistentDescriptorSet::start(cursor_pipeline.clone(), 0)
                .add_sampled_image(cursor_texture.clone(), cursor_sampler.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        // FIXME: those descriptor are used by many pipeline other than
        // draw1_pipeline. Is it OK ?
        let draw1_view_descriptor_set_pool =
            FixedSizeDescriptorSetsPool::new(draw1_pipeline.clone(), 0);
        let draw1_dynamic_descriptor_set_pool =
            FixedSizeDescriptorSetsPool::new(draw1_pipeline.clone(), 0);
        let imgui_matrix_descriptor_set_pool =
            FixedSizeDescriptorSetsPool::new(imgui_pipeline.clone(), 0);

        let eraser2_descriptor_set = Arc::new(
            PersistentDescriptorSet::start(eraser2_pipeline.clone(), 0)
                .add_buffer(tmp_erased_buffer.clone())
                .unwrap()
                .add_buffer(erased_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let (debug_arrow_vertex_buffer, debug_arrow_future) =
            primitives::load_debug_arrow(queue.clone());

        now(device.clone())
            .join(cursor_tex_future)
            .join(colors_buf_future)
            .join(fullscreen_vertex_buffer_future)
            .join(cursor_vertex_buffer_future)
            .join(primitives_future)
            .join(debug_arrow_future)
            .flush()
            .unwrap();

        Graphics {
            physical,
            data: Data {
                fullscreen_vertex_buffer,
                swapchain,
                images,
                device,
                queue,
                render_pass,
                second_render_pass,
                draw1_pipeline,
                draw1_eraser_pipeline,
                draw1_hud_pipeline,
                draw2_pipeline,
                debug_pipeline,
                framebuffer,
                second_framebuffers,
                dim,
                cursor_tex_dim,
                view_uniform_buffer,
                primitives_vertex_buffers,
                cursor_descriptor_set,
                cursor_pipeline,
                imgui_pipeline,
                cursor_vertex_buffer,
                imgui_descriptor_set,
                draw1_view_descriptor_set_pool,
                draw1_dynamic_descriptor_set_pool,
                imgui_matrix_descriptor_set_pool,
                eraser1_pipeline,
                eraser2_pipeline,
                eraser1_descriptor_set_0,
                eraser1_descriptor_set_1,
                eraser2_descriptor_set,
                tmp_erased_buffer,
                draw2_descriptor_set_0,
                draw2_descriptor_set_1,
                world_uniform_static_buffer,
                world_uniform_buffer,
                debug_arrow_world_uniform_buffer,
                erased_buffer,
                debug_arrow_vertex_buffer,
            },
        }
    }

    pub fn recreate(&mut self, window: &'a ::vulkano_win::Window, imgui: &mut ::imgui::ImGui) {
        let recreate;
        loop {
            // TODO: Sleep and max number of try
            let dimensions = window
                .surface()
                .capabilities(self.physical)
                .expect("failed to get surface capabilities")
                .current_extent
                .unwrap_or([1024, 768]);
            match self.data.swapchain.recreate_with_dimension(dimensions) {
                Err(::vulkano::swapchain::SwapchainCreationError::UnsupportedDimensions) => (),
                r @ _ => {
                    recreate = Some(r);
                    break;
                }
            }
        }

        let (new_swapchain, new_images) = recreate.unwrap().unwrap();
        self.data.dim = new_images[0].dimensions();
        self.data.images = new_images;
        self.data.swapchain = new_swapchain;

        let (framebuffer, second_framebuffers, eraser1_descriptor_set_0, draw2_descriptor_set_0, imgui_descriptor_set) =
            Graphics::framebuffers_and_descriptors(
                self.data.device.clone(),
                self.data.queue.clone(),
                &self.data.images,
                &self.data.render_pass,
                &self.data.second_render_pass,
                &self.data.eraser1_pipeline,
                &self.data.draw2_pipeline,
                &self.data.imgui_pipeline,
                imgui,
            );

        self.data.framebuffer = framebuffer;
        self.data.second_framebuffers = second_framebuffers;
        self.data.eraser1_descriptor_set_0 = eraser1_descriptor_set_0;
        self.data.draw2_descriptor_set_0 = draw2_descriptor_set_0;
        self.data.imgui_descriptor_set = imgui_descriptor_set;
    }
}
