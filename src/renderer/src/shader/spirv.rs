use spirv_reflect::types::image::ReflectFormat;

use crate::{VertexFormat, WgpuFrom};

impl WgpuFrom<ReflectFormat> for VertexFormat {
    fn from(val: ReflectFormat) -> Self {
        match val {
            ReflectFormat::R32_UINT => VertexFormat::Uint32,
            ReflectFormat::R32_SINT => VertexFormat::Sint32,
            ReflectFormat::R32_SFLOAT => VertexFormat::Float32,
            ReflectFormat::R32G32_UINT => VertexFormat::Uint32x2,
            ReflectFormat::R32G32_SINT => VertexFormat::Sint32x2,
            ReflectFormat::R32G32_SFLOAT => VertexFormat::Float32x2,
            ReflectFormat::R32G32B32_UINT => VertexFormat::Uint32x3,
            ReflectFormat::R32G32B32_SINT => VertexFormat::Sint32x2,
            ReflectFormat::R32G32B32_SFLOAT => VertexFormat::Float32x3,
            ReflectFormat::R32G32B32A32_UINT => VertexFormat::Uint32x4,
            ReflectFormat::R32G32B32A32_SINT => VertexFormat::Sint32x4,
            ReflectFormat::R32G32B32A32_SFLOAT => VertexFormat::Float32x4,
            ReflectFormat::Undefined => panic!("Should not happen, I guess?"),
        }
    }
}
