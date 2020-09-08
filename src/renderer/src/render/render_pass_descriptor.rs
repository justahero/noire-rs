use crate::{Color, Operations, TextureId, SamplerId};

#[derive(Debug)]
pub enum TextureAttachment {
    Input(String),
}

#[derive(Debug)]
pub struct RenderPassColorAttachmentDescriptor {
    /// The color attachment
    pub attachment: TextureAttachment,
    /// The resolve target for this color attachment, if set
    pub resolve_target: TextureAttachment,
    /// Operations will be performed on this color attachment
    pub ops: Operations<Color>,
}

#[derive(Debug)]
pub struct RenderPassDepthStencilAttachmentDescriptor {
    /// The view or texture attachment
    pub attachment: TextureAttachment,
    /// What operation will be performed on the depth part of the attachment
    pub depth_ops: Option<Operations<f32>>,
    /// Waht operation will be performed on the stencil part of the attachment
    pub stencil_ops: Option<Operations<f32>>,
}

#[derive(Debug)]
pub struct RenderPassDescriptor {
    /// The color attachments of this render pass
    pub color_attachments: Vec<RenderPassColorAttachmentDescriptor>,
    /// The depth and stencil attach of the render pass, if set
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachmentDescriptor>,
    /// Number of samples for the render pass
    pub samples: u32,
}
