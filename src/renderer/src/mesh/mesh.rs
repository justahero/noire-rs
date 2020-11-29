use std::{borrow::Cow, fmt::Display};

use crate::{PrimitiveTopology, VertexBufferDescriptor, VertexFormat};

#[derive(Debug)]
pub enum VertexAttributeValues {
    Float(Vec<f32>),
    Float2(Vec<[f32; 2]>),
    Float3(Vec<[f32; 3]>),
    Float4(Vec<[f32; 4]>),
}

impl VertexAttributeValues {
    pub fn len(&self) -> usize {
        match *self {
            VertexAttributeValues::Float(ref values) => values.len(),
            VertexAttributeValues::Float2(ref values) => values.len(),
            VertexAttributeValues::Float3(ref values) => values.len(),
            VertexAttributeValues::Float4(ref values) => values.len(),
        }
    }

    /// Returns true if there are no vert, false otherwise
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the internal bytes
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            VertexAttributeValues::Float(values) => bytemuck::cast_slice(values.as_slice()),
            VertexAttributeValues::Float2(values) => bytemuck::cast_slice(values.as_slice()),
            VertexAttributeValues::Float3(values) => bytemuck::cast_slice(values.as_slice()),
            VertexAttributeValues::Float4(values) => bytemuck::cast_slice(values.as_slice()),
        }
    }
}

impl From<&VertexAttributeValues> for VertexFormat {
    fn from(values: &VertexAttributeValues) -> Self {
        match values {
            VertexAttributeValues::Float(_) => VertexFormat::Float,
            VertexAttributeValues::Float2(_) => VertexFormat::Float2,
            VertexAttributeValues::Float3(_) => VertexFormat::Float3,
            VertexAttributeValues::Float4(_) => VertexFormat::Float4,
        }
    }
}

impl From<Vec<f32>> for VertexAttributeValues {
    fn from(vec: Vec<f32>) -> Self {
        VertexAttributeValues::Float(vec)
    }
}

#[derive(Debug)]
pub struct VertexAttribute {
    /// The name of the vertex attribute
    pub name: Cow<'static, str>,
    /// The list of vertex attribute values
    pub values: VertexAttributeValues,
}

impl VertexAttribute {
    /// Sets the positions for the mesh
    pub fn positions(positions: Vec<[f32; 3]>) -> Self {
        VertexAttribute {
            name: "positions".into(),
            values: VertexAttributeValues::Float3(positions),
        }
    }

    /// Sets the normals for the mesh
    pub fn normals(normals: Vec<[f32; 3]>) -> Self {
        VertexAttribute {
            name: "normals".into(),
            values: VertexAttributeValues::Float3(normals),
        }
    }

    /// Sets the tex coords / uv coords for all vertices
    pub fn texcoords(texcoords: Vec<[f32; 2]>) -> Self {
        VertexAttribute {
            name: "texcoords".into(),
            values: VertexAttributeValues::Float2(texcoords),
        }
    }

    /// Returns the vertex size of this attribute
    pub fn vertex_size(&self) -> usize {
        let format: VertexFormat = (&self.values).into();
        format.size() as usize
    }
}

impl Display for VertexAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&VertexAttribute> for VertexFormat {
    fn from(attribute: &VertexAttribute) -> VertexFormat {
        (&attribute.values).into()
    }
}

#[derive(Debug)]
pub enum Indices {
    /// u16 based indices list
    U16(Vec<u16>),
    /// u32 based indices list
    U32(Vec<u32>),
}

impl Indices {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Indices::U16(v) => bytemuck::cast_slice(v),
            Indices::U32(v) => bytemuck::cast_slice(v),
        }
    }

    /// Returns the indices count
    pub fn len(&self) -> u32 {
        match self {
            Indices::U16(v) => v.len() as u32,
            Indices::U32(v) => v.len() as u32,
        }
    }
}

/// A Mesh struct that contains vertices, normals, tex coords.
#[derive(Debug)]
pub struct Mesh {
    /// Type of primitive topology
    pub topology: PrimitiveTopology,
    /// The list of vertex attributes
    pub attributes: Vec<VertexAttribute>,
    /// The list of optional indices referencing vertex data
    pub indices: Option<Indices>,
}

impl Mesh {
    /// Creates a new Mesh instance
    pub fn new(topology: PrimitiveTopology) -> Self {
        Mesh {
            topology,
            attributes: Vec::new(),
            indices: None,
        }
    }

    /// Returns the count of all vertices
    pub fn vertex_count(&self) -> usize {
        let mut vertex_count: Option<usize> = None;

        for attribute in self.attributes.iter() {
            let attributes_size = attribute.values.len();
            if let Some(previous) = vertex_count {
                assert_eq!(
                    previous,
                    attributes_size,
                    "Attribute {} has different vertex count ({}) than other attributes", previous, attribute
                );
            }
            vertex_count = Some(attributes_size);
        }

        vertex_count.unwrap_or(0)
    }

    /// Returns the stride of a single vertex
    pub fn vertex_stride(&self) -> usize {
        self.attributes
            .iter()
            .map(|attribute| attribute.vertex_size())
            .sum()
    }

    /// Returns all bytes as u8 vector
    pub fn vertex_data(&self) -> Vec<u8> {
        let vertex_count = self.vertex_count();
        let vertex_size = self.vertex_stride();
        let mut interleaved_data = vec![0; vertex_count * vertex_size];

        let mut attribute_offset = 0;
        for attribute in self.attributes.iter() {
            let vertex_format: VertexFormat = attribute.into();
            let attribute_size = vertex_format.size() as usize;
            let attribute_bytes = attribute.values.get_bytes();

            for (vertex_index, attribute_bytes) in attribute_bytes.chunks_exact(attribute_size).enumerate() {
                let offset = vertex_index * vertex_size + attribute_offset;
                interleaved_data[offset..offset + attribute_size].copy_from_slice(attribute_bytes);
            }

            attribute_offset += attribute_size;
        }

        interleaved_data
    }

    /// Returns vertex buffer descriptor
    pub fn vertex_buffer_descriptor(&self) -> VertexBufferDescriptor {
        let vertex_formats = self.attributes
            .iter()
            .map(|attribute| attribute.into())
            .collect::<Vec<VertexFormat>>();

        VertexBufferDescriptor::new(vertex_formats)
    }
}

pub mod shape {
    use crate::{Indices, Mesh, PrimitiveTopology, VertexAttribute};

    pub struct Cube {
        /// Half the side length of the cube
        pub size: f32,
    }

    impl Cube {
        pub fn new(size: f32) -> Self {
            Cube { size }
        }
    }

    impl Default for Cube {
        fn default() -> Self {
            Cube { size: 1.0, }
        }
    }

    impl From<Cube> for Mesh {
        fn from(cube: Cube) -> Self {
            let half = cube.size / 2.0;
            let vertices = vec![
                [-half, -half,  half],
                [ half, -half,  half],
                [ half,  half,  half],
                [-half,  half,  half],

                [-half, -half, -half],
                [-half,  half, -half],
                [ half,  half, -half],
                [ half, -half, -half],

                [-half,  half, -half],
                [-half,  half,  half],
                [ half,  half,  half],
                [ half,  half, -half],

                [-half, -half, -half],
                [ half, -half, -half],
                [ half, -half,  half],
                [-half, -half,  half],

                [ half, -half, -half],
                [ half,  half, -half],
                [ half,  half,  half],
                [ half, -half,  half],

                [-half, -half, -half],
                [-half, -half,  half],
                [-half,  half,  half],
                [-half,  half, -half],
            ];

            let normals = vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],

                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],

                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],

                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],

                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],

                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
            ];

            let texcoords = vec![
                [0.0,  0.0],
                [1.0,  0.0],
                [1.0,  1.0],
                [0.0,  1.0],

                [0.0,  0.0],
                [1.0,  0.0],
                [1.0,  1.0],
                [0.0,  1.0],

                [0.0,  0.0],
                [1.0,  0.0],
                [1.0,  1.0],
                [0.0,  1.0],

                [0.0,  0.0],
                [1.0,  0.0],
                [1.0,  1.0],
                [0.0,  1.0],

                [0.0,  0.0],
                [1.0,  0.0],
                [1.0,  1.0],
                [0.0,  1.0],

                [0.0,  0.0],
                [1.0,  0.0],
                [1.0,  1.0],
                [0.0,  1.0],
            ];

            let indices = Indices::U32(vec![
                0,  1,  2,    0,  2,  3,  // Front face
                4,  5,  6,    4,  6,  7,  // Back face
                8,  9, 10,    8, 10, 11,  // Top face
               12, 13, 14,   12, 14, 15,  // Bottom face
               16, 17, 18,   16, 18, 19,  // Right face
               20, 21, 22,   20, 22, 23   // Left face
            ]);

            Mesh {
                topology: PrimitiveTopology::TriangleList,
                attributes: vec![
                    VertexAttribute::positions(vertices),
                    VertexAttribute::normals(normals),
                    VertexAttribute::texcoords(texcoords),
                ],
                indices: Some(indices),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Mesh, shape};

    #[test]
    fn cube_vertex() {
        let cube: Mesh = shape::Cube::new(1.0).into();

        assert_eq!(cube.vertex_count(), 3 * 8);
        assert_eq!(cube.vertex_stride(), (3 + 3 + 2) * std::mem::size_of::<f32>());
        assert_eq!(cube.vertex_data().len(), (3 + 3 + 2) * 8 * std::mem::size_of::<f32>());
    }
}
