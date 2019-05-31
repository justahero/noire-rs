pub struct Cube {
    pub vertices: Vec<f32>,
    pub texcoords: Vec<f32>,
    // pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

fn create_vertices(size: f32) -> Vec<f32> {
    let half = size / 2.0;

    vec![
      -half, -half,  half,
       half, -half,  half,
       half,  half,  half,
      -half,  half,  half,

      -half, -half, -half,
      -half,  half, -half,
       half,  half, -half,
       half, -half, -half,

      -half,  half, -half,
      -half,  half,  half,
       half,  half,  half,
       half,  half, -half,

      -half, -half, -half,
       half, -half, -half,
       half, -half,  half,
      -half, -half,  half,

       half, -half, -half,
       half,  half, -half,
       half,  half,  half,
       half, -half,  half,

      -half, -half, -half,
      -half, -half,  half,
      -half,  half,  half,
      -half,  half, -half,
    ]
}

fn create_texcoords() -> Vec<f32> {
    vec![
      0.0,  0.0,
      1.0,  0.0,
      1.0,  1.0,
      0.0,  1.0,

      0.0,  0.0,
      1.0,  0.0,
      1.0,  1.0,
      0.0,  1.0,

      0.0,  0.0,
      1.0,  0.0,
      1.0,  1.0,
      0.0,  1.0,

      0.0,  0.0,
      1.0,  0.0,
      1.0,  1.0,
      0.0,  1.0,

      0.0,  0.0,
      1.0,  0.0,
      1.0,  1.0,
      0.0,  1.0,

      0.0,  0.0,
      1.0,  0.0,
      1.0,  1.0,
      0.0,  1.0,
    ]
}

fn create_indices() -> Vec<u32> {
    vec![
       0,  1,  2,    0,  2,  3,  // Front face
       4,  5,  6,    4,  6,  7,  // Back face
       8,  9, 10,    8, 10, 11,  // Top face
      12, 13, 14,   12, 14, 15,  // Bottom face
      16, 17, 18,   16, 18, 19,  // Right face
      20, 21, 22,   20, 22, 23   // Left face
    ]
}

impl Cube {
    pub fn create(size: f32) -> Self {
        Cube {
            vertices: create_vertices(size),
            texcoords: create_texcoords(),
            indices: create_indices(),
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
}
