#[derive(Debug)]
pub struct VertexAttributeDescriptor {
    pub offset: u64,
    pub location: u32,
    pub format: VertexFormat,
}

impl From<VertexAttributeDescriptor> for wgpu::VertexAttributeDescriptor {
    fn from(val: VertexAttributeDescriptor) -> Self {
        wgpu::VertexAttributeDescriptor {
            offset: val.offset,
            format: val.format.into(),
            shader_location: val.location,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Uchar2,
    Uchar4,
    Char2,
    Char4,
    Uchar2Norm,
    Uchar4Norm,
    Char2Norm,
    Char4Norm,
    Ushort2,
    Ushort4,
    Short2,
    Short4,
    Ushort2Norm,
    Ushort4Norm,
    Short2Norm,
    Short4Norm,
    Half2,
    Half4,
    Float,
    Float2,
    Float3,
    Float4,
    Uint,
    Uint2,
    Uint3,
    Uint4,
    Int,
    Int2,
    Int3,
    Int4,
}

impl From<VertexFormat> for wgpu::VertexFormat {
    fn from(val: VertexFormat) -> Self {
        match val {
            VertexFormat::Uchar2 => wgpu::VertexFormat::Uchar2,
            VertexFormat::Uchar4 => wgpu::VertexFormat::Uchar4,
            VertexFormat::Char2 => wgpu::VertexFormat::Char2,
            VertexFormat::Char4 => wgpu::VertexFormat::Char4,
            VertexFormat::Uchar2Norm => wgpu::VertexFormat::Uchar2Norm,
            VertexFormat::Uchar4Norm => wgpu::VertexFormat::Uchar4Norm,
            VertexFormat::Char2Norm => wgpu::VertexFormat::Char2Norm,
            VertexFormat::Char4Norm => wgpu::VertexFormat::Char4Norm,
            VertexFormat::Ushort2 => wgpu::VertexFormat::Ushort2,
            VertexFormat::Ushort4 => wgpu::VertexFormat::Ushort4,
            VertexFormat::Short2 => wgpu::VertexFormat::Short2,
            VertexFormat::Short4 => wgpu::VertexFormat::Short4,
            VertexFormat::Ushort2Norm => wgpu::VertexFormat::Ushort2Norm,
            VertexFormat::Ushort4Norm => wgpu::VertexFormat::Ushort4Norm,
            VertexFormat::Short2Norm => wgpu::VertexFormat::Short2Norm,
            VertexFormat::Short4Norm => wgpu::VertexFormat::Short4Norm,
            VertexFormat::Half2 => wgpu::VertexFormat::Half2,
            VertexFormat::Half4 => wgpu::VertexFormat::Half4,
            VertexFormat::Float => wgpu::VertexFormat::Float,
            VertexFormat::Float2 => wgpu::VertexFormat::Float2,
            VertexFormat::Float3 => wgpu::VertexFormat::Float3,
            VertexFormat::Float4 => wgpu::VertexFormat::Float4,
            VertexFormat::Uint => wgpu::VertexFormat::Uint,
            VertexFormat::Uint2 => wgpu::VertexFormat::Uint2,
            VertexFormat::Uint3 => wgpu::VertexFormat::Uint3,
            VertexFormat::Uint4 => wgpu::VertexFormat::Uint4,
            VertexFormat::Int => wgpu::VertexFormat::Int,
            VertexFormat::Int2 => wgpu::VertexFormat::Int2,
            VertexFormat::Int3 => wgpu::VertexFormat::Int3,
            VertexFormat::Int4 => wgpu::VertexFormat::Int4,
        }
    }
}
