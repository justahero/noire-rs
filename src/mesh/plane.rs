pub struct Plane {
    pub vertices: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

fn create_vertices(width: f32, height: f32) -> Vec<f32> {
    let xsize = width / 2.0;
    let ysize = height / 2.0;
    vec![
        -xsize, 0.0,  ysize,
         xsize, 0.0,  ysize,
         xsize, 0.0, -ysize,
        -xsize, 0.0, -ysize,
    ]
}

fn create_texcoords() -> Vec<f32> {
    vec![
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
    ]
}

fn create_normals() -> Vec<f32> {
    vec![
        0.0,  1.0,  0.0,
        0.0,  1.0,  0.0,
        0.0,  1.0,  0.0,
        0.0,  1.0,  0.0,
    ]
}

fn create_indices() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3]
}

impl Plane {
    pub fn create(width: f32, height: f32) -> Self {
        Plane {
            vertices: create_vertices(width, height),
            texcoords: create_texcoords(),
            normals: create_normals(),
            indices: create_indices(),
        }
    }
}
