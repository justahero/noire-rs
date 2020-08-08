use gl::types::GLenum;

use super::{Bindable, RenderError, Texture};

/// A general purpose Framebuffer to store pixel data into
/// This is a good resource to learn more about Framebuffers, https://open.gl/framebuffers
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

impl FrameBuffer {
    /// Create a new instance of a Frame Buffer
    pub fn create() -> Result<Self, RenderError> {
        let mut id = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut id);
        }

        Ok(FrameBuffer { id })
    }

    /// Set Texture to this Framebuffer
    ///
    /// ## Arguments
    ///
    /// * `texture` - The texture to attach
    pub fn attach_texture(&mut self, texture: &Texture) -> Result<&mut Self, RenderError> {
        self.set_texture(gl::COLOR_ATTACHMENT0, texture.target, texture.id)
    }

    /// Detaches the texture from the Framebuffer
    pub fn detach_texture(&mut self, texture: &Texture) -> Result<&mut Self, RenderError> {
        self.set_texture(gl::COLOR_ATTACHMENT0, texture.target, 0)
    }

    /// Set Depth Texture to this Framebuffer
    ///
    /// ## Argumnets
    ///
    /// * `texture` - the depth Texture instance
    pub fn set_depth_buffer(&mut self, texture: &Texture) -> Result<&mut Self, RenderError> {
        self.set_texture(gl::DEPTH_ATTACHMENT, texture.target, texture.id)
    }

    /// Attaches or detaches a texture or renderbuffer to or from this Framebuffer
    /// A convenience wrapper function around 'FramebufferTexture2D'
    fn set_texture(&mut self, attachment: GLenum, target: GLenum, id: u32) -> Result<&mut Self, RenderError> {
        self.bind();

        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, target, id, 0);
        }

        check_status()?;

        self.unbind();

        Ok(self)
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
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
