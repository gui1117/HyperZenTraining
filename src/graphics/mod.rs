use vulkano::framebuffer::RenderPassDesc;

use std::sync::Arc;
use std::iter;

pub mod shader;
pub mod render_pass;

lazy_static! {
    pub static ref GROUP_COUNTER: GroupCounter = GroupCounter::new();
}

pub struct GroupCounter {
    counter: ::std::sync::atomic::AtomicUsize,
}

impl GroupCounter {
    fn new() -> Self {
        GroupCounter { counter: ::std::sync::atomic::AtomicUsize::new(1) }
    }

    pub fn next(&self) -> u32 {
        self.counter.fetch_add(
            1,
            ::std::sync::atomic::Ordering::Relaxed,
        ) as u32
    }
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

#[derive(Clone)]
pub struct Data {
    pub device: Arc<::vulkano::device::Device>,
    pub queue: Arc<::vulkano::device::Queue>,
    pub swapchain: Arc<::vulkano::swapchain::Swapchain>,
    pub images: Vec<Arc<::vulkano::image::swapchain::SwapchainImage>>,
    pub depth_buffer_attachment: Arc<::vulkano::image::attachment::AttachmentImage>,
    pub tmp_image_attachment: Arc<::vulkano::image::attachment::AttachmentImage>,
    pub plane_vertex_buffer: Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<[Vertex]>>,
    pub pyramid_vertex_buffer: Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<[Vertex]>>,
    pub fullscreen_vertex_buffer: Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<[SecondVertex]>>,
    pub render_pass: Arc<::vulkano::framebuffer::RenderPass<render_pass::CustomRenderPassDesc>>,
    pub second_render_pass: Arc<::vulkano::framebuffer::RenderPass<render_pass::SecondCustomRenderPassDesc>>,
    pub pipeline: Arc<::vulkano::pipeline::GraphicsPipeline<::vulkano::pipeline::vertex::SingleBufferDefinition<Vertex>, Box<::vulkano::descriptor::PipelineLayoutAbstract + Sync + Send>, ::Arc<::vulkano::framebuffer::RenderPass<render_pass::CustomRenderPassDesc>>>>,
    pub second_pipeline: Arc<::vulkano::pipeline::GraphicsPipeline<::vulkano::pipeline::vertex::SingleBufferDefinition<SecondVertex>, Box<::vulkano::descriptor::PipelineLayoutAbstract + Sync + Send>, ::Arc<::vulkano::framebuffer::RenderPass<render_pass::SecondCustomRenderPassDesc>>>>,
    pub framebuffer: Arc<::vulkano::framebuffer::Framebuffer<Arc<::vulkano::framebuffer::RenderPass<render_pass::CustomRenderPassDesc>>, (((), Arc<::vulkano::image::AttachmentImage>), Arc<::vulkano::image::AttachmentImage>)>>,
    pub second_framebuffers: Vec<Arc<::vulkano::framebuffer::Framebuffer<Arc<::vulkano::framebuffer::RenderPass<render_pass::SecondCustomRenderPassDesc>>, ((), Arc<::vulkano::image::SwapchainImage>)>>>,
    pub width: u32,
    pub height: u32,
    pub view_uniform_buffer: ::vulkano::buffer::cpu_pool::CpuBufferPool<::graphics::shader::vs::ty::View>,
    pub tmp_image_set: Arc<::vulkano::descriptor::descriptor_set::PersistentDescriptorSet<Arc<::vulkano::pipeline::GraphicsPipeline<::vulkano::pipeline::vertex::SingleBufferDefinition<::graphics::SecondVertex>, Box<::vulkano::descriptor::PipelineLayoutAbstract + Sync + Send>, Arc<::vulkano::framebuffer::RenderPass<::graphics::render_pass::SecondCustomRenderPassDesc>>>>, (((), ::vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg<Arc<::vulkano::image::AttachmentImage>>), ::vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler)>>
}

pub struct Graphics<'a> {
    pub physical: ::vulkano::instance::PhysicalDevice<'a>,
    pub data: Data,
}

impl<'a> Graphics<'a> {
    pub fn new(window: &'a ::vulkano_win::Window) -> Graphics<'a> {
        //TODO: read config and save device
        let physical = ::vulkano::instance::PhysicalDevice::enumerate(&window.surface().instance())
            .next()
            .expect("no device available");

        let queue_family = physical
            .queue_families()
            .find(|&q| {
                q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
            })
            .expect("couldn't find a graphical queue family");

        let (device, mut queues) = {
            let device_ext = ::vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                ..::vulkano::device::DeviceExtensions::none()
            };

            ::vulkano::device::Device::new(
                physical,
                physical.supported_features(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned(),
            ).expect("failed to create device")
        };

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let caps = window.surface().capabilities(physical).expect(
                "failed to get surface capabilities",
            );

            let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
            let image_usage = ::vulkano::image::ImageUsage {
                sampled: true,
                color_attachment: true,
                ..::vulkano::image::ImageUsage::none()
            };

            ::vulkano::swapchain::Swapchain::new(
                device.clone(),
                window.surface().clone(),
                caps.min_image_count,
                ::vulkano::format::Format::B8G8R8A8Srgb,
                dimensions,
                1,
                image_usage,
                &queue,
                ::vulkano::swapchain::SurfaceTransform::Identity,
                ::vulkano::swapchain::CompositeAlpha::Opaque,
                ::vulkano::swapchain::PresentMode::Fifo,
                true,
                None,
            ).expect("failed to create swapchain")
        };

        let width = images[0].dimensions()[0];
        let height = images[0].dimensions()[1];

        let depth_buffer_attachment = ::vulkano::image::attachment::AttachmentImage::transient(
            device.clone(),
            images[0].dimensions(),
            ::vulkano::format::Format::D16Unorm,
        ).unwrap();

        let tmp_image_attachment = {
            let usage = ::vulkano::image::ImageUsage {
                color_attachment: true,
                sampled: true,
                ..::vulkano::image::ImageUsage::none()
            };
            ::vulkano::image::attachment::AttachmentImage::with_usage(
                device.clone(),
                images[0].dimensions(),
                ::vulkano::format::Format::R32Uint,
                usage,
            ).unwrap()
        };

        let plane_vertex_buffer = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
            device.clone(),
            ::vulkano::buffer::BufferUsage::vertex_buffer(),
            [
                Vertex { position: [-1.0, -1.0, 0.0] },
                Vertex { position: [1.0, -1.0, 0.0] },
                Vertex { position: [-1.0, 1.0, 0.0] },
                Vertex { position: [1.0, 1.0, 0.0] },
                Vertex { position: [-1.0, 1.0, 0.0] },
                Vertex { position: [1.0, -1.0, 0.0] },
            ].iter()
                .cloned(),
        ).expect("failed to create buffer");

        let pyramid_vertex_buffer = ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
            device.clone(),
            ::vulkano::buffer::BufferUsage::vertex_buffer(),
            [
                Vertex { position: [-1.0, -1.0, -1.0] },
                Vertex { position: [1.0, -1.0, -1.0] },
                Vertex { position: [-1.0, 1.0, -1.0] },

                Vertex { position: [1.0, 1.0, -1.0] },
                Vertex { position: [1.0, -1.0, -1.0] },
                Vertex { position: [-1.0, 1.0, -1.0] },

                Vertex { position: [-1.0, -1.0, -1.0] },
                Vertex { position: [-1.0, 1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },

                Vertex { position: [-1.0, 1.0, -1.0] },
                Vertex { position: [1.0, 1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },

                Vertex { position: [1.0, 1.0, -1.0] },
                Vertex { position: [1.0, -1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },

                Vertex { position: [1.0, -1.0, -1.0] },
                Vertex { position: [-1.0, -1.0, -1.0] },
                Vertex { position: [0.0, 0.0, 1.0] },
            ].iter()
                .cloned(),
        ).expect("failed to create buffer");

        let fullscreen_vertex_buffer =
            ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::from_iter(
                device.clone(),
                ::vulkano::buffer::BufferUsage::vertex_buffer(),
                [
                    SecondVertex { position: [-1.0f32, -1.0] },
                    SecondVertex { position: [1.0, -1.0] },
                    SecondVertex { position: [-1.0, 1.0] },
                    SecondVertex { position: [1.0, 1.0] },
                    SecondVertex { position: [-1.0, 1.0] },
                    SecondVertex { position: [1.0, -1.0] },
                ].iter()
                    .cloned(),
            ).expect("failed to create buffer");

        let vs = shader::vs::Shader::load(device.clone()).expect("failed to create shader module");
        let fs = shader::fs::Shader::load(device.clone()).expect("failed to create shader module");

        let second_vs = shader::second_vs::Shader::load(device.clone()).expect(
            "failed to create shader module",
        );
        let second_fs = shader::second_fs::Shader::load(device.clone()).expect(
            "failed to create shader module",
        );

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

        let pipeline = Arc::new(
            ::vulkano::pipeline::GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(vs.main_entry_point(), ())
                .viewports(iter::once(::vulkano::pipeline::viewport::Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0..1.0,
                    dimensions: [width as f32, height as f32],
                }))
                .fragment_shader(fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .render_pass(
                    ::vulkano::framebuffer::Subpass::from(render_pass.clone(), 0).unwrap(),
                )
                .build(device.clone())
                .unwrap(),
        );

        let second_pipeline = Arc::new(
            ::vulkano::pipeline::GraphicsPipeline::start()
                .vertex_input_single_buffer::<SecondVertex>()
                .vertex_shader(second_vs.main_entry_point(), ())
                .triangle_list()
                .viewports(iter::once(::vulkano::pipeline::viewport::Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0..1.0,
                    dimensions: [width as f32, height as f32],
                }))
                .fragment_shader(second_fs.main_entry_point(), ())
                .render_pass(
                    ::vulkano::framebuffer::Subpass::from(second_render_pass.clone(), 0).unwrap(),
                )
                .build(device.clone())
                .unwrap(),
        );

        let framebuffer = Arc::new(
            ::vulkano::framebuffer::Framebuffer::start(render_pass.clone())
                .add(tmp_image_attachment.clone())
                .unwrap()
                .add(depth_buffer_attachment.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let second_framebuffers = images
            .iter()
            .map(|image| {
                Arc::new(
                    ::vulkano::framebuffer::Framebuffer::start(second_render_pass.clone())
                        .add(image.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let view_uniform_buffer =
            ::vulkano::buffer::cpu_pool::CpuBufferPool::<::graphics::shader::vs::ty::View>::new(
                device.clone(),
                ::vulkano::buffer::BufferUsage::uniform_buffer(),
            );

        //TODO: maybe use simple instead of persistent
        let tmp_image_set = Arc::new(
            ::vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
                second_pipeline.clone(),
                0,
            ).add_sampled_image(
                tmp_image_attachment.clone(),
                // Sampler::simple_repeat_linear_no_mipmap(graphics.device.clone()),
                ::vulkano::sampler::Sampler::unnormalized(
                    device.clone(),
                    ::vulkano::sampler::Filter::Nearest,
                    ::vulkano::sampler::UnnormalizedSamplerAddressMode::ClampToEdge,
                    ::vulkano::sampler::UnnormalizedSamplerAddressMode::ClampToEdge,
                ).unwrap(),
            )
                .unwrap()
                .build()
                .unwrap(),
        );

        Graphics {
            physical,
            data: Data {
                plane_vertex_buffer,
                pyramid_vertex_buffer,
                fullscreen_vertex_buffer,
                depth_buffer_attachment,
                tmp_image_attachment,
                swapchain,
                images,
                device,
                queue,
                render_pass,
                second_render_pass,
                pipeline,
                second_pipeline,
                framebuffer,
                second_framebuffers,
                width,
                height,
                tmp_image_set,
                view_uniform_buffer,
            },
        }
    }
}
