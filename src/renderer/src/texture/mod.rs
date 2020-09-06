#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    // 8 bit formats, unpacked
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,

    // 16 bit formats, unpacked
    R16Uint,
    R16Sint,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,

    // 32 bit formats, unpacked
    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,

    // packed 32 bit formats
    Rgb10a2Unorm,
    Rg11b10Float,

    // packed 64 bit formats
    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Float,

    // 128 bit formats, unpacked
    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,

    // Depth Stencil formats
    Depth32Float,
    Depth24Plus,
    Depth24PlusStencil8,
}

impl From<TextureFormat> for wgpu::TextureFormat {
    fn from(format: TextureFormat) -> Self {
        todo!()
    }
}
