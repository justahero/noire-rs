use std::num::NonZeroU32;

use crate::{TextureDescriptor, TextureFormat, TextureViewDimension};

/// Specifies a texture
#[derive(Debug)]
pub struct Texture {
    /// Descriptor to the Texture
    pub descriptor: TextureDescriptor,
    /// The wgpu texture
    pub texture: wgpu::Texture,
    /// The wgpu texture view
    pub view: wgpu::TextureView,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Creates a new texture with the given descriptor
    pub fn new(
        descriptor: TextureDescriptor,
        device: &wgpu::Device,
    ) -> Self {
        let texture: wgpu::Texture = device.create_texture(&descriptor.clone().into());
        let view_descriptor = TextureViewDescriptor::create_from_texture(&descriptor);
        let view = texture.create_view(&view_descriptor.into());

        Self {
            descriptor: descriptor.clone(),
            texture,
            view,
        }
    }

    /// Returns the width of the texutre
    pub fn width(&self) -> u32 {
        self.descriptor.size.width
    }

    /// Returns the height of the texture
    pub fn height(&self) -> u32 {
        self.descriptor.size.height
    }

    /// Format of the texture
    pub fn texture_format(&self) -> TextureFormat {
        self.descriptor.texture_format
    }

    /// Allowed usage of the texture
    pub fn usage(&self) -> TextureUsage {
        self.descriptor.usage
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureAspect {
    All,
    StencilOnly,
    DepthOnly,
}

impl From<TextureAspect> for wgpu::TextureAspect {
    fn from(val: TextureAspect) -> Self {
        match val {
            TextureAspect::All => wgpu::TextureAspect::All,
            TextureAspect::StencilOnly => wgpu::TextureAspect::StencilOnly,
            TextureAspect::DepthOnly => wgpu::TextureAspect::DepthOnly,
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct TextureUsage: u32 {
        /// Allows a texture to be the source
        const COPY_SRC = 1;
        /// Allows a texture to be the destination
        const COPY_DST = 2;
        /// Allows a texture to be a sampled texture in a bind group
        const SAMPLED = 4;
        /// Allows a texture to be a storage texture in a bind group
        const STORAGE = 8;
        /// Allows a texture to be an output attachment of a render pass
        const OUTPUT_ATTACHMENT = 16;
        /// None of the bits set
        const NONE = 0;
    }
}

impl From<TextureUsage> for wgpu::TextureUsage {
    fn from(val: TextureUsage) -> Self {
        wgpu::TextureUsage::from_bits(val.bits()).unwrap()
    }
}

#[derive(Debug)]
pub struct TextureViewDescriptor {
    /// Debug label of the texture view
    pub label: Option<String>,
    /// Format of the texture view
    pub format: Option<TextureFormat>,
    /// Dimension of the texture view
    pub dimension: Option<TextureViewDimension>,
    /// Aspect ratio of the texture
    pub aspect: TextureAspect,
    /// Base mip level
    pub base_mip_level: u32,
    /// Mip level count
    pub mip_level_count: Option<NonZeroU32>,
    /// Base Array Layer
    pub base_array_layer: u32,
    /// Layer count
    pub array_layer_count: Option<NonZeroU32>,
}

impl TextureViewDescriptor {
    pub fn create_from_texture(descriptor: &TextureDescriptor) -> Self {
        Self {
            label: None,
            format: Some(descriptor.texture_format),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: NonZeroU32::new(1),
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(1),
        }
    }
}

impl<'a> From<TextureViewDescriptor> for wgpu::TextureViewDescriptor<'a> {
    fn from(descriptor: TextureViewDescriptor) -> Self {
        Self {
            label: None,
            format: descriptor.format.map(|f| f.into()),
            dimension: descriptor.dimension.map(|d| d.into()),
            aspect: descriptor.aspect.into(),
            base_mip_level: descriptor.base_mip_level,
            mip_level_count: descriptor.mip_level_count,
            base_array_layer: descriptor.base_array_layer,
            array_layer_count: descriptor.array_layer_count,
        }
    }
}
