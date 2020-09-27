use std::{any::Any, any::TypeId, cell::Ref, collections::HashMap};

/// Marker trait to store instances associated with a specific type
pub trait Resource: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Resource for T {}

/*
pub(crate) struct ResourceData {
    // TODO figure out how to store sized traits that can be cast to 
    // specific structs again
    pub list: Vec<Box<dyn Any>>,
}

impl ResourceData {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
        }
    }
}
*/

/// Holds the list of all relevant resources for an App
pub struct Resources {
    pub data: HashMap<TypeId, Vec<Box<dyn Resource>>>,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Resources {
    /// Returns a new empty Resources container
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert a new resource type
    pub fn insert<T: Resource>(&mut self, mut resource: T) {
        let type_id = TypeId::of::<T>();
        self.data.entry(type_id).or_insert_with(|| {
            /*
            let mut items = Vec::new();
            items.push(Box::new(resource));
            items
            */
            let mut items = Vec::new();
            items
        });
    }

    pub fn get<T: Resource>(&self) -> Option<Ref<'_, T>> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|data| {
                // Ref::new(&data, 0).ok()
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::Resources;

    pub struct Point2 {
        pub x: f32,
        pub y: f32,
    }

    #[test]
    fn it_inserts_resources() {
        let resources = Resources::new();
        resources.insert::<Point2>(Point2 { x: 0.0, y: 0.0 });
        resources.insert::<String>("hello_world".to_string());

        assert_eq!(2, resources.data.len());
    }
}
