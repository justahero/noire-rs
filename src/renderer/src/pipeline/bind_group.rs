use std::num::NonZeroU32;

use crate::{BindingDescriptor, ShaderStage, UniformProperty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingType {
    /// Binding type is a uniform buffer
    Uniform {
        dynamic: bool,
        property: UniformProperty,
    },
    Unknown,
}

impl BindingType {
    pub fn get_size(&self) -> Option<u64> {
        match self {
            BindingType::Uniform { property, .. } => Some(property.get_size()),
            _ => None,
        }
    }
}

impl From<&BindingType> for wgpu::BindingType {
    fn from(binding_type: &BindingType) -> Self {
        match binding_type {
            BindingType::Uniform{ dynamic, .. } => wgpu::BindingType::UniformBuffer{
                dynamic: *dynamic,
                min_binding_size: binding_type.get_size().and_then(wgpu::BufferSize::new),
            },
            _ => panic!("Binding type currently not supported"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupDescriptor {
    /// Index of the bind group
    pub index: u32,
    /// The
    pub bindings: Vec<BindingDescriptor>,
}

impl BindGroupDescriptor {
    pub fn new(index: u32, bindings: Vec<BindingDescriptor>) -> Self {
        Self {
            index,
            bindings,
        }
    }

    /// Finds a given Binding Descriptor in this Bind Group Descriptor
    pub fn contains(&self, other: &BindingDescriptor) -> bool {
        if let Some(binding) = self.bindings.iter().find(|rhs| rhs.index == other.index) {
            if binding.binding_type == other.binding_type && binding.name == other.name {
                true
            } else {
                panic!("Binding {} in BindGroup {} is not consistent across shader types: ", binding.index, self.index);
            }
        } else {
            false
        }
    }
}

/// Describes a single binding inside a BindGroup
///
/// For example in shader
///
/// layout(set = 0, binding = 1) uniform;
///
#[derive(Debug)]
pub struct BindGroupLayoutEntry {
    /// Debug label
    pub label: Option<String>,
    /// The Binding index, must match shader index and unique inside a BindGroupLayout
    pub binding: u32,
    /// Which shader can see this binding
    pub visibility: ShaderStage,
    /// The type of binding
    pub binding_type: BindingType,
    /// Indices if this entry is an array, must be 1 or greater
    pub count: Option<NonZeroU32>,
}

impl From<&BindGroupLayoutEntry> for wgpu::BindGroupLayoutEntry {
    fn from(entry: &BindGroupLayoutEntry) -> Self {
        Self {
            binding: entry.binding,
            visibility: entry.visibility.into(),
            // TODO currently set one type directly
            ty: wgpu::BindingType::UniformBuffer {
                dynamic: true,
                min_binding_size: None,
            },
            count: entry.count,
        }
    }
}

#[derive(Debug)]
pub struct BindGroupLayoutDescriptor {
    /// Debug label of the bind group layout
    pub label: Option<String>,
    /// Array of entrie in this BindGroupLayout
    pub entries: Vec<BindGroupLayoutEntry>,
}
