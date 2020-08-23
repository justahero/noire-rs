use gl::types::GLenum;

use super::{Bindable, RenderError, Texture, RenderBuffer};
use image::DynamicImage;
use std::ffi::c_void;
use crate::math::Color;

#[derive(Clone, Copy)]
pub enum Attachment {
    Back,
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl From<Attachment> for gl::types::GLenum {
    fn from(attachment: Attachment) -> Self {
        match attachment {
            Attachment::Back => gl::BACK,
            Attachment::Color(index) => gl::COLOR_ATTACHMENT0 + index,
            Attachment::Depth => gl::DEPTH_ATTACHMENT,
            Attachment::Stencil => gl::STENCIL_ATTACHMENT,
            Attachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

/// A general purpose Framebuffer to store pixel data into
///
/// ## Resources
///
/// * Introduction into Framebuffers: https://open.gl/framebuffers
/// * OpenGL Framebuffer: https://www.khronos.org/opengl/wiki/Framebuffer
pub struct FrameBuffer {
    pub id: u32,
}

/// Checks the status of the Frame Buffer, return error with message or nothing
fn check_status() -> Result<(), RenderError> {
    unsafe {
        let result = gl::CheckFramebufferStatus(gl::FRAMEBUFFER) as u32;

        if result != gl::FRAMEBUFFER_COMPLETE {
            return Err(RenderError{ message: status_error(result) });
        }
    }

    Ok(())
}

/// Returns the error string based on given Framebuffer error code
fn status_error(error: u32) -> String {
    match error {
        gl::FRAMEBUFFER_UNSUPPORTED => "Framebuffer is not supported".to_string(),
        gl::FRAMEBUFFER_UNDEFINED => "Framebuffer is undefined".to_string(),
        gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => "Framebuffer incomplete attachment".to_string(),
        gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => "Framebuffer incomplete missing attachement".to_string(),
        gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => "Framebuffer incomplete draw buffer".to_string(),
        gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => "Framebuffer incomplete read buffer".to_string(),
        gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => "Framebuffer incomplete multi sample".to_string(),
        gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => "Framebuffer incomplete layer targets".to_string(),
        _ => format!("Unknown error: {}", error),
    }
}

/// Small wrapper function to blit pixel data from a read framebuffer to a destination framebuffer,
/// copies a block of pixel data from one framebuffer to the other, while allowing different sized
/// framebuffer objects, it's scaled up or down accordingly.
///
/// This function is also useful to copy data from a MSAA sampled framebuffer to a non-sampled FBO.
///
/// For convenience, the blit function assumes the full size of the framebuffers are used.
pub fn blit(read_buffer: &FrameBuffer, write_buffer: &FrameBuffer, width: u32, height: u32) -> Result<(), RenderError> {
    let mask = gl::COLOR_BUFFER_BIT;

    unsafe {
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, read_buffer.id);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, write_buffer.id);
    }

    unsafe {
        gl::BlitFramebuffer(
            0,
            0,
            width as i32,
            height as i32,
            0,
            0,
            width as i32,
            height as i32,
            mask,
            gl::NEAREST,
        );
    }

    Ok(())
}

/// Copies the content of the frame buffer (BACK) into an image.
///
/// Check the following article for more details on how to read pixel data from
/// Framebuffer to store it in an image file
/// https://tonyfinn.com/capturing-screenshots-with-rust-opengl.html
///
pub fn copy_frame_buffer_to_image(width: u32, height: u32) -> DynamicImage {
    let mut image = DynamicImage::new_rgba8(width, height);
    let pixel_data = image.as_mut_rgba8().unwrap();

    unsafe {
        let ptr = pixel_data.as_mut_ptr() as *mut c_void;

        gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
        gl::ReadPixels(
            0, 0,
            width as i32, height as i32,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            ptr,
        );
    }

    image
}

impl FrameBuffer {
    pub const BACK: FrameBuffer = FrameBuffer { id: 0 };

    /// Create a new instance of a Frame Buffer
    pub fn create() -> Result<Self, RenderError> {
        let mut id = 0;

        unsafe { gl::GenFramebuffers(1, &mut id); }

        Ok(FrameBuffer { id })
    }

    /// Checks if the current attachments / configs are valid for this Framebuffer
    /// Returns a result with possible error message
    pub fn valid(&self) -> Result<(), RenderError> {
        check_status()
    }

    /// Clears the frame buffer color part
    pub fn clear(&self, color: Color) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::ClearBufferfv(gl::COLOR, 0, &color.rgba_array()[0]);
        }
    }

    /// Set Texture to this Framebuffer
    ///
    /// ## Arguments
    ///
    /// * `texture` - The texture to attach
    /// * `index` - The color attachment slot
    pub fn attach_texture(&mut self, texture: &Texture, index: u32) -> Result<(), RenderError> {
        self.set_texture(Attachment::Color(index), texture.target, texture.id)
    }

    /// Detaches the texture from the Framebuffer
    pub fn detach_texture(&mut self, texture: &Texture, index: u32) -> Result<(), RenderError> {
        self.set_texture(Attachment::Color(index), texture.target, 0)
    }

    /// Attaches a Renderbuffer to this Framebuffer
    pub fn attach_renderbuffer(&mut self, attachment: Attachment, buffer: &RenderBuffer) -> Result<(), RenderError> {
        self.set_renderbuffer(attachment, buffer.id)
    }

    /// Detaches any Renderbuffer from this Framebuffer
    /// **Note**, for now it only uses the first color attachment slot
    pub fn detach_renderbuffer(&mut self, attachment: Attachment) -> Result<(), RenderError> {
        self.set_renderbuffer(attachment, 0)
    }

    /// Set Depth Texture to this Framebuffer
    ///
    /// ## Argumnets
    ///
    /// * `texture` - the depth Texture instance
    pub fn set_depth_buffer(&mut self, texture: &Texture) -> Result<(), RenderError> {
        self.set_texture(Attachment::Depth, texture.target, texture.id)
    }

    /// Attaches or detaches a texture or renderbuffer to or from this Framebuffer
    /// A convenience wrapper function around 'FramebufferTexture2D'
    fn set_texture(&mut self, attachment: Attachment, target: GLenum, id: u32) -> Result<(), RenderError> {
        self.bind();

        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment.into(), target, id, 0);
        }

        check_status()?;

        self.unbind();

        Ok(())
    }

    /// Attaches or detaches the given renderbuffer
    fn set_renderbuffer(&mut self, attachment: Attachment, id: u32) -> Result<(), RenderError> {
        self.bind();

        unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, attachment.into(), gl::RENDERBUFFER, id);
        }

        check_status()?;

        self.unbind();

        Ok(())
    }
}

impl Bindable for FrameBuffer {
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
        self
    }

    fn unbind(&mut self) -> &mut Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut id);
        }

        self.id == (id as u32)
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteFramebuffers(1, &self.id);
            }
            self.id = 0;
        }
    }
}
