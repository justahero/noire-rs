use super::{Format, Bindable, RenderError, opengl::get_error, Size};

unsafe fn generate_buffer() -> u32 {
    let mut id = 0;
    gl::GenRenderbuffers(1, &mut id);
    id
}

/// A RenderBuffer is a buffer meant for offscreen rendering.
/// It is a storage object (buffer) containing a single image of a renderable format
///
pub struct RenderBuffer {
    /// The associated OpenGL id (handle) to the RenderBuffer object
    pub id: u32,
    /// The color / pixel format
    pub format: Format,
    /// The width of the Render Buffer
    pub width: u32,
    /// The height of the Render Buffer
    pub height: u32,
}

impl RenderBuffer {
    /// Creates a new RenderBuffer object with given format and dimensions
    pub fn new(format: Format, width: u32, height: u32) -> Result<Self, RenderError> {
        let id = unsafe { generate_buffer() };

        let mut buffer = Self {
            id,
            format,
            width,
            height,
        };

        buffer.allocate(width, height)?;

        Ok(buffer)
    }

    /// Allocate memory to hold 2 dimensional image / pixel data
    ///
    /// ## Useful resources:
    /// * OpenGL Anti Aliasing https://learnopengl.com/Advanced-OpenGL/Anti-Aliasing
    ///
    pub fn allocate(&mut self, width: u32, height: u32) -> Result<(), RenderError> {
        self.bind();

        unsafe {
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
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
        self.id = 0;
    }
}
