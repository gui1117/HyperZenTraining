pub struct CustomRenderPassDesc;

unsafe impl ::vulkano::framebuffer::RenderPassDesc for CustomRenderPassDesc {
    #[inline]
    fn num_attachments(&self) -> usize {
        2
    }

    #[inline]
    fn attachment_desc(
        &self,
        id: usize,
    ) -> Option<::vulkano::framebuffer::LayoutAttachmentDescription> {
        match id {
            0 => Some(::vulkano::framebuffer::LayoutAttachmentDescription {
                format: ::vulkano::format::Format::R32Uint,
                samples: 1,
                load: ::vulkano::framebuffer::LoadOp::Clear,
                store: ::vulkano::framebuffer::StoreOp::Store,
                stencil_load: ::vulkano::framebuffer::LoadOp::Clear,
                stencil_store: ::vulkano::framebuffer::StoreOp::Store,
                initial_layout: ::vulkano::image::ImageLayout::Undefined,
                final_layout: ::vulkano::image::ImageLayout::ColorAttachmentOptimal,
            }),
            1 => Some(::vulkano::framebuffer::LayoutAttachmentDescription {
                format: ::vulkano::format::Format::D16Unorm,
                samples: 1,
                load: ::vulkano::framebuffer::LoadOp::Clear,
                store: ::vulkano::framebuffer::StoreOp::DontCare,
                stencil_load: ::vulkano::framebuffer::LoadOp::Clear,
                stencil_store: ::vulkano::framebuffer::StoreOp::DontCare,
                initial_layout: ::vulkano::image::ImageLayout::Undefined,
                final_layout: ::vulkano::image::ImageLayout::DepthStencilAttachmentOptimal,
            }),
            _ => None,
        }
    }

    #[inline]
    fn num_subpasses(&self) -> usize {
        1
    }

    #[inline]
    fn subpass_desc(&self, id: usize) -> Option<::vulkano::framebuffer::LayoutPassDescription> {
        match id {
            0 => Some(::vulkano::framebuffer::LayoutPassDescription {
                color_attachments: vec![(0, ::vulkano::image::ImageLayout::ColorAttachmentOptimal)],
                depth_stencil: Some((
                    1,
                    ::vulkano::image::ImageLayout::DepthStencilAttachmentOptimal,
                )),
                input_attachments: vec![],
                resolve_attachments: vec![],
                preserve_attachments: vec![],
            }),
            _ => None,
        }
    }

    #[inline]
    fn num_dependencies(&self) -> usize {
        0
    }

    #[inline]
    fn dependency_desc(
        &self,
        _id: usize,
    ) -> Option<::vulkano::framebuffer::LayoutPassDependencyDescription> {
        None
    }
}

unsafe impl ::vulkano::framebuffer::RenderPassDescClearValues<Vec<::vulkano::format::ClearValue>>
    for CustomRenderPassDesc {
    fn convert_clear_values(
        &self,
        values: Vec<::vulkano::format::ClearValue>,
    ) -> Box<Iterator<Item = ::vulkano::format::ClearValue>> {
        // FIXME: safety checks
        Box::new(values.into_iter())
    }
}

pub struct SecondCustomRenderPassDesc;

unsafe impl ::vulkano::framebuffer::RenderPassDesc for SecondCustomRenderPassDesc {
    #[inline]
    fn num_attachments(&self) -> usize {
        1
    }

    #[inline]
    fn attachment_desc(
        &self,
        id: usize,
    ) -> Option<::vulkano::framebuffer::LayoutAttachmentDescription> {
        match id {
            0 => Some(::vulkano::framebuffer::LayoutAttachmentDescription {
                format: ::vulkano::format::Format::B8G8R8A8Srgb,
                samples: 1,
                load: ::vulkano::framebuffer::LoadOp::DontCare,
                store: ::vulkano::framebuffer::StoreOp::Store,
                stencil_load: ::vulkano::framebuffer::LoadOp::DontCare,
                stencil_store: ::vulkano::framebuffer::StoreOp::Store,
                initial_layout: ::vulkano::image::ImageLayout::Undefined,
                final_layout: ::vulkano::image::ImageLayout::ColorAttachmentOptimal,
            }),
            _ => None,
        }
    }

    #[inline]
    fn num_subpasses(&self) -> usize {
        1
    }

    #[inline]
    fn subpass_desc(&self, id: usize) -> Option<::vulkano::framebuffer::LayoutPassDescription> {
        match id {
            0 => Some(::vulkano::framebuffer::LayoutPassDescription {
                color_attachments: vec![(0, ::vulkano::image::ImageLayout::ColorAttachmentOptimal)],
                depth_stencil: None,
                input_attachments: vec![],
                resolve_attachments: vec![],
                preserve_attachments: vec![],
            }),
            _ => None,
        }
    }

    #[inline]
    fn num_dependencies(&self) -> usize {
        0
    }

    #[inline]
    fn dependency_desc(
        &self,
        _id: usize,
    ) -> Option<::vulkano::framebuffer::LayoutPassDependencyDescription> {
        None
    }
}

unsafe impl ::vulkano::framebuffer::RenderPassDescClearValues<Vec<::vulkano::format::ClearValue>>
    for SecondCustomRenderPassDesc {
    fn convert_clear_values(
        &self,
        values: Vec<::vulkano::format::ClearValue>,
    ) -> Box<Iterator<Item = ::vulkano::format::ClearValue>> {
        // FIXME: safety checks
        Box::new(values.into_iter())
    }
}
