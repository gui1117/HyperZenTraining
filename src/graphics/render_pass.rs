use vulkano::framebuffer::{
    LoadOp,
    StoreOp,
    LayoutAttachmentDescription,
    LayoutPassDescription,
    LayoutPassDependencyDescription,
    RenderPassDesc,
    RenderPassDescClearValues,
};
use vulkano::image::ImageLayout;
use vulkano::format::{Format, ClearValue};
pub struct CustomRenderPassDesc;

unsafe impl RenderPassDesc for CustomRenderPassDesc {
    #[inline]
    fn num_attachments(&self) -> usize {
        2
    }

    #[inline]
    fn attachment_desc(
        &self,
        id: usize,
    ) -> Option<LayoutAttachmentDescription> {
        match id {
            0 => Some(LayoutAttachmentDescription {
                format: Format::R16G16Uint,
                samples: 1,
                load: LoadOp::Clear,
                store: StoreOp::Store,
                stencil_load: LoadOp::Clear,
                stencil_store: StoreOp::Store,
                initial_layout: ImageLayout::Undefined,
                final_layout: ImageLayout::ColorAttachmentOptimal,
            }),
            1 => Some(LayoutAttachmentDescription {
                format: Format::D16Unorm,
                samples: 1,
                load: LoadOp::Clear,
                store: StoreOp::DontCare,
                stencil_load: LoadOp::Clear,
                stencil_store: StoreOp::DontCare,
                initial_layout: ImageLayout::Undefined,
                final_layout: ImageLayout::DepthStencilAttachmentOptimal,
            }),
            _ => None,
        }
    }

    #[inline]
    fn num_subpasses(&self) -> usize {
        1
    }

    #[inline]
    fn subpass_desc(&self, id: usize) -> Option<LayoutPassDescription> {
        match id {
            0 => Some(LayoutPassDescription {
                color_attachments: vec![(0, ImageLayout::ColorAttachmentOptimal)],
                depth_stencil: Some((
                    1,
                    ImageLayout::DepthStencilAttachmentOptimal,
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
    ) -> Option<LayoutPassDependencyDescription> {
        None
    }
}

unsafe impl RenderPassDescClearValues<Vec<ClearValue>>
    for CustomRenderPassDesc {
    fn convert_clear_values(
        &self,
        values: Vec<ClearValue>,
    ) -> Box<Iterator<Item = ClearValue>> {
        // FIXME: safety checks
        Box::new(values.into_iter())
    }
}

pub struct SecondCustomRenderPassDesc;

unsafe impl RenderPassDesc for SecondCustomRenderPassDesc {
    #[inline]
    fn num_attachments(&self) -> usize {
        1
    }

    #[inline]
    fn attachment_desc(
        &self,
        id: usize,
    ) -> Option<LayoutAttachmentDescription> {
        match id {
            0 => Some(LayoutAttachmentDescription {
                format: Format::B8G8R8A8Srgb,
                samples: 1,
                load: LoadOp::DontCare,
                store: StoreOp::Store,
                stencil_load: LoadOp::DontCare,
                stencil_store: StoreOp::Store,
                initial_layout: ImageLayout::Undefined,
                final_layout: ImageLayout::ColorAttachmentOptimal,
            }),
            _ => None,
        }
    }

    #[inline]
    fn num_subpasses(&self) -> usize {
        1
    }

    #[inline]
    fn subpass_desc(&self, id: usize) -> Option<LayoutPassDescription> {
        match id {
            0 => Some(LayoutPassDescription {
                color_attachments: vec![(0, ImageLayout::ColorAttachmentOptimal)],
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
    ) -> Option<LayoutPassDependencyDescription> {
        None
    }
}

unsafe impl RenderPassDescClearValues<Vec<ClearValue>>
    for SecondCustomRenderPassDesc {
    fn convert_clear_values(
        &self,
        values: Vec<ClearValue>,
    ) -> Box<Iterator<Item = ClearValue>> {
        // FIXME: safety checks
        Box::new(values.into_iter())
    }
}
