use mesh::Node;

pub struct Scene {
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Scene::default()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            nodes: Vec::new()
        }
    }
}
