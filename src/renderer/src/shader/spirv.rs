use spirv_reflect::types::image::ReflectFormat;

use crate::{VertexFormat, WgpuFrom};

impl WgpuFrom<ReflectFormat> for VertexFormat {
    fn from(val: ReflectFormat) -> Self {
        match val {
            ReflectFormat::R32_UINT => VertexFormat::Uint,
            ReflectFormat::R32_SINT => VertexFormat::Int,
            ReflectFormat::R32_SFLOAT => VertexFormat::Float,
            ReflectFormat::R32G32_UINT => VertexFormat::Uint2,
            ReflectFormat::R32G32_SINT => VertexFormat::Int2,
            ReflectFormat::R32G32_SFLOAT => VertexFormat::Float2,
            ReflectFormat::R32G32B32_UINT => VertexFormat::Uint3,
            ReflectFormat::R32G32B32_SINT => VertexFormat::Int3,
            ReflectFormat::R32G32B32_SFLOAT => VertexFormat::Float3,
            ReflectFormat::R32G32B32A32_UINT => VertexFormat::Uint4,
            ReflectFormat::R32G32B32A32_SINT => VertexFormat::Int4,
            ReflectFormat::R32G32B32A32_SFLOAT => VertexFormat::Float4,
            ReflectFormat::Undefined => panic!("Should not happen, I guess?"),
        }
    }
}
