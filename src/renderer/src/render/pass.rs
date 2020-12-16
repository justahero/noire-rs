use crate::{Color, Operations, Surface, Texture, Window};

#[derive(Debug)]
pub struct RenderPassColorAttachmentDescriptor<'a> {
    /// The color attachment, e.g. Surface
    pub attachment: &'a mut Surface,
    /// The resolve target, e.g. Texture
    pub resolve_target: Option<&'a Texture>,
    /// The clear color operation
    pub ops: Operations<Color>,
}

#[derive(Debug)]
pub struct RenderPassDepthStencilAttachmentDescriptor<'a> {
    /// The depth stencil attachment
    pub attachment: &'a Texture,
    /// The depth operations
    pub depth_ops: Option<Operations<f32>>,
    /// The stencil Operations
    pub stencil_ops: Option<Operations<u32>>,
}

#[derive(Debug)]
pub struct PassDescriptor<'a> {
    /// The list of color attachments, e.g. surfaces
    pub color_attachments: Vec<RenderPassColorAttachmentDescriptor<'a>>,
    /// Optional depth stencil attmachment
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachmentDescriptor<'a>>,
}

impl<'a> Default for PassDescriptor<'a> {
    fn default() -> Self {
        Self {
            color_attachments: Vec::new(),
            depth_stencil_attachment: None,
        }
    }
}

impl<'a> From<&'a mut Window> for PassDescriptor<'a> {
    fn from(window: &'a mut Window) -> Self {
        let color = Color::BLACK;
        let color_attachment = RenderPassColorAttachmentDescriptor {
            attachment: &mut window.surface,
            resolve_target: None,
            ops: Operations::new(color),
        };
        let depth_stencil_attachment = RenderPassDepthStencilAttachmentDescriptor {
            attachment: &window.depth_buffer,
            depth_ops: Some(Operations::clear(0.0)),
            stencil_ops: None,
        };

        Self {
            color_attachments: vec![color_attachment],
            depth_stencil_attachment: Some(depth_stencil_attachment),
        }
    }
}
