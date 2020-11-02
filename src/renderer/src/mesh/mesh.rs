use std::borrow::Cow;

use crate::PrimitiveTopology;

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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
}

#[derive(Debug)]
pub enum Indices {
    /// u16 based indices list
    U16(Vec<u16>),
    /// u32 based indices list
    U32(Vec<u32>),
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
}

pub mod shape {
    use crate::{Indices, Mesh, PrimitiveTopology, VertexAttribute};

    pub struct Cube {
        /// Half the side length of the cube
        pub size: f32,
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

/*
    /// Type of primitive topology
    pub topology: PrimitiveTopology,
    /// The list of vertex attributes
    pub attributes: Vec<VertexAttribute>,
    /// The list of optional indices referencing vertex data
    pub indices: Option<Indices>,
*/
