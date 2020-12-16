use crate::TextureFormat;

pub struct ColorStateDescriptor {
    pub format: TextureFormat,
    pub alpha_blend: BlendDescriptor,
    pub color_blend: BlendDescriptor,
    pub write_mask: ColorWrite,
}

impl Default for ColorStateDescriptor {
    fn default() -> Self {
        Self {
            format: TextureFormat::Bgra8UnormSrgb,
            color_blend: BlendDescriptor::COLOR_BLEND,
            alpha_blend: BlendDescriptor::ALPHA_BLEND,
            write_mask: ColorWrite::ALL,
        }
    }
}

impl From<ColorStateDescriptor> for wgpu::ColorStateDescriptor {
    fn from(val: ColorStateDescriptor) -> Self {
        wgpu::ColorStateDescriptor {
            format: val.format.into(),
            alpha_blend: val.alpha_blend.into(),
            color_blend: val.color_blend.into(),
            write_mask: val.write_mask.into(),
        }
    }
}

#[derive(Debug)]
pub struct BlendDescriptor {
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub operation: BlendOperation,
}

impl From<BlendDescriptor> for wgpu::BlendDescriptor {
    fn from(val: BlendDescriptor) -> Self {
        wgpu::BlendDescriptor {
            src_factor: val.src_factor.into(),
            dst_factor: val.dst_factor.into(),
            operation: val.operation.into(),
        }
    }
}

impl BlendDescriptor {
    pub const COLOR_BLEND: Self = Self {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    };

    pub const ALPHA_BLEND: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    };

    pub const REPLACE: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::Zero,
        operation: BlendOperation::Add,
    };
}

#[derive(Debug, Clone, Copy)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
}

impl From<BlendFactor> for wgpu::BlendFactor {
    fn from(val: BlendFactor) -> Self {
        match val {
            BlendFactor::Zero => wgpu::BlendFactor::Zero,
            BlendFactor::One => wgpu::BlendFactor::One,
            BlendFactor::SrcColor => wgpu::BlendFactor::SrcColor,
            BlendFactor::OneMinusSrcColor => wgpu::BlendFactor::OneMinusSrcColor,
            BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
            BlendFactor::DstColor => wgpu::BlendFactor::DstColor,
            BlendFactor::OneMinusDstColor => wgpu::BlendFactor::OneMinusDstColor,
            BlendFactor::DstAlpha => wgpu::BlendFactor::DstAlpha,
            BlendFactor::OneMinusDstAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
            BlendFactor::SrcAlphaSaturated => wgpu::BlendFactor::SrcAlphaSaturated,
            BlendFactor::BlendColor => wgpu::BlendFactor::BlendColor,
            BlendFactor::OneMinusBlendColor => wgpu::BlendFactor::OneMinusBlendColor,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl From<BlendOperation> for wgpu::BlendOperation {
    fn from(val: BlendOperation) -> Self {
        match val {
            BlendOperation::Add => wgpu::BlendOperation::Add,
            BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
            BlendOperation::ReverseSubtract => wgpu::BlendOperation::ReverseSubtract,
            BlendOperation::Min => wgpu::BlendOperation::Min,
            BlendOperation::Max => wgpu::BlendOperation::Max,
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorWrite: u32 {
        const RED = 1;
        const GREEN = 2;
        const BLUE = 4;
        const ALPHA = 8;
        const COLOR = 7;
        const ALL = 15;
    }
}

impl Default for ColorWrite {
    fn default() -> Self {
        ColorWrite::ALL
    }
}

impl From<ColorWrite> for wgpu::ColorWrite {
    fn from(val: ColorWrite) -> Self {
        wgpu::ColorWrite::from_bits(val.bits()).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl From<CompareFunction> for wgpu::CompareFunction {
    fn from(val: CompareFunction) -> Self {
        match val {
            CompareFunction::Never => wgpu::CompareFunction::Never,
            CompareFunction::Less => wgpu::CompareFunction::Less,
            CompareFunction::Equal => wgpu::CompareFunction::Equal,
            CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
            CompareFunction::Greater => wgpu::CompareFunction::Greater,
            CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
            CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
            CompareFunction::Always => wgpu::CompareFunction::Always,
        }
    }
}
