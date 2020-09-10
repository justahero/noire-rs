use crate::{TextureFormat, TextureUsage};

#[derive(Debug, Copy, Clone)]
pub struct Extend3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl From<Extend3d> for wgpu::Extent3d {
    fn from(val: Extend3d) -> Self {
        Self {
            width: val.width,
            height: val.height,
            depth: val.depth,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureDimension {
    D1 = 1,
    D2 = 2,
    D3 = 3,
}

impl From<TextureDimension> for wgpu::TextureDimension {
    fn from(val: TextureDimension) -> Self {
        match val {
            TextureDimension::D1 => wgpu::TextureDimension::D1,
            TextureDimension::D2 => wgpu::TextureDimension::D2,
            TextureDimension::D3 => wgpu::TextureDimension::D3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

impl From<TextureViewDimension> for wgpu::TextureViewDimension {
    fn from(val: TextureViewDimension) -> Self {
        match val {
            TextureViewDimension::D1 => wgpu::TextureViewDimension::D1,
            TextureViewDimension::D2 => wgpu::TextureViewDimension::D2,
            TextureViewDimension::D2Array => wgpu::TextureViewDimension::D2Array,
            TextureViewDimension::Cube => wgpu::TextureViewDimension::Cube,
            TextureViewDimension::CubeArray=> wgpu::TextureViewDimension::CubeArray,
            TextureViewDimension::D3 => wgpu::TextureViewDimension::D3,
        }
    }
}

#[derive(Debug)]
pub struct TextureDescriptor {
    /// Debug label of the texture
    pub label: Option<String>,
    /// Size of the texture
    pub size: Extend3d,
    /// Mip count of texture, for a texture with no extra mips this must be 1
    pub mip_level_count: u32,
    /// Sample count of the texture
    pub sample_count: u32,
    /// The dimension of the texture
    pub dimension: TextureDimension,
    /// The format of the texture
    pub texture_format: TextureFormat,
    /// Allowed usages of the texture
    pub usage: TextureUsage,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            label: None,
            size: Extend3d { width: 512, height: 512, depth: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            texture_format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsage::OUTPUT_ATTACHMENT
                | TextureUsage::SAMPLED
                | TextureUsage::COPY_SRC,
        }
    }
}

impl TextureDescriptor {
    /// Create a new descriptor for a 2d texture
    pub fn texture_2d(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            size: Extend3d { width, height, depth: 1 },
            texture_format: format,
            .. Default::default()
        }
    }

    /// Creates a new depth 2d texture for depth buffer
    pub fn depth(width: u32, height: u32) -> Self {
        Self {
            size: Extend3d { width, height, depth: 1 },
            texture_format: TextureFormat::Depth32Float,
            .. Default::default()
        }
    }

    /// Creates a depth stencil 2d texture
    pub fn depth_stencil(width: u32, height: u32) -> Self {
        Self {
            size: Extend3d { width, height, depth: 1 },
            texture_format: TextureFormat::Depth24PlusStencil8,
            .. Default::default()
        }
    }
}

impl<'a> From<&TextureDescriptor> for wgpu::TextureDescriptor<'a> {
    fn from(descriptor: &TextureDescriptor) -> Self {
        Self {
            label: None,
            size: descriptor.size.into(),
            mip_level_count: descriptor.mip_level_count,
            sample_count: descriptor.sample_count,
            dimension: descriptor.dimension.into(),
            format: descriptor.texture_format.into(),
            usage: descriptor.usage.into(),
        }
    }
}
