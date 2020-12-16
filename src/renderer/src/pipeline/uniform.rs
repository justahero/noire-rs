/// A Uniform is a variable that can be bound as part of bind group to a shader
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uniform {
    /// The name of the uniform
    pub name: String,
    /// The property / type of the uniform
    pub property: UniformProperty,
}

impl Uniform {
    pub fn new(name: &str, property: UniformProperty) -> Self {
        Self {
            name: name.to_string(),
            property,
        }
    }

    pub fn get_size(&self) -> u64 {
        self.property.get_size()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniformProperty {
    UInt,
    Int,
    IVec2,
    Float,
    UVec4,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Struct(Vec<Uniform>),
    Array(Box<Uniform>, usize),
}

impl UniformProperty {
    pub fn get_size(&self) -> u64 {
        match self {
            UniformProperty::UInt => 4,
            UniformProperty::Int => 4,
            UniformProperty::IVec2 => 4 * 2,
            UniformProperty::Float => 4,
            UniformProperty::UVec4 => 4 * 4,
            UniformProperty::Vec2 => 4 * 2,
            UniformProperty::Vec3 => 4 * 3,
            UniformProperty::Vec4 => 4 * 4,
            UniformProperty::Mat3 => 4 * 4 * 3,
            UniformProperty::Mat4 => 4 * 4 * 4,
            UniformProperty::Struct(properties) => {
                properties.iter().map(|p| p.get_size()).sum()
            }
            UniformProperty::Array(uniform, length) => uniform.get_size() * *length as u64,
        }
    }
}
