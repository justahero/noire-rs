use super::{Format, Bindable, RenderError, opengl::get_error, Size};

/// A RenderBuffer is a buffer meant for offscreen rendering.
/// It is a storage object (buffer) containing a single image of a renderable format
///
pub struct RenderBuffer {
    /// The associated OpenGL id (handle) to the RenderBuffer object
    pub id: u32,
    /// The color / pixel format
    pub format: Format,
    /// The MSAA sampling factor
    pub msaa: u32,
}

impl RenderBuffer {
    /// Creates a new RenderBuffer object.
    /// It has not yet allocated any data storage.
    pub fn new(format: Format, msaa: u32) -> Self {
        let mut id = 0;

        unsafe { gl::GenRenderbuffers(1, &mut id); }

        Self {
            id,
            msaa,
            format,
        }
    }

    /// Allocate memory to hold 2 dimensional image / pixel data
    ///
    /// ## Useful resources:
    /// * OpenGL Anti Aliasing https://learnopengl.com/Advanced-OpenGL/Anti-Aliasing
    ///
    pub fn allocate(&mut self, width: u32, height: u32) -> Result<(), RenderError> {
        self.bind();

        let msaa = 0;

        unsafe {
            gl::RenderbufferStorageMultisample(
                gl::RENDERBUFFER,
                msaa,
                self.format.into(),
                width as i32,
                height as i32,
            );
        }

        get_error().map_err(|message| RenderError::new(message))?;

        self.unbind();

        Ok(())
    }

    /// Returns the dimensions of the Renderbuffer
    pub fn size(&self) -> Size<u32> {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl Bindable for RenderBuffer {
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
        self
    }

    fn unbind(&mut self) -> &mut Self {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::RENDERBUFFER_BINDING, &mut id);
        }
        self.id == (id as u32)
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.id);
        }
    }
}
