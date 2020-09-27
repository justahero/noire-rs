// This Resources struct is extracted from the bevy_hecs ECS project (https://crates.io/crates/bevy_hecs)
// It is stripped down for now to only deal with one type of resource index.
//
// The `Resources` struct uses the `Archetype` functionality to store hetereogeneous data in a HashMap.
// indexed by assocated type ids of the struct type to be stored under.
//
use std::{any::TypeId, cmp::Ordering, collections::HashMap};

/// Marker trait to store instances associated with a specific type
pub trait Resource: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Resource for T {}

pub struct ResourceData {
    archetype: bevy_hecs::Archetype,
    default_index: Option<usize>,
}

/// Holds the list of all relevant resources for an App
pub struct Resources {
    data: HashMap<TypeId, ResourceData>,
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
    ///
    /// This function is adapted from the bevy_hecs project to insert an arbitrary object via the Resource type id.
    ///
    pub fn insert<T: Resource>(&mut self, mut resource: T) {
        let type_id = TypeId::of::<T>();
        let data = self.data.entry(type_id).or_insert_with(|| {
            let mut types = Vec::new();
            types.push(bevy_hecs::TypeInfo::of::<T>());
            ResourceData {
                archetype: bevy_hecs::Archetype::new(types),
                default_index: None,
            }
        });

        let archetype = &mut data.archetype;
        let mut added = false;

        let index = *data
            .default_index
            .get_or_insert_with(|| {
                added = true;
                archetype.len()
            });

        match index.cmp(&archetype.len()) {
            Ordering::Equal => {
                unsafe { archetype.allocate(bevy_hecs::Entity::new(index as u32)) };
            }
            Ordering::Greater => panic!("Attempted to access index {} beyond {}", index, archetype.len()),
            Ordering::Less => (),
        }

        unsafe {
            let resource_ptr = (&mut resource as *mut T).cast::<u8>();
            archetype.put_dynamic(
                resource_ptr,
                type_id,
                core::mem::size_of::<T>(),
                index,
                added,
            );
            std::mem::forget(resource);
        }
    }

    pub fn get<T: Resource>(&self) -> Option<bevy_hecs::Ref<'_, T>> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|data| unsafe {
                bevy_hecs::Ref::new(&data.archetype, 0).ok()
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
        let mut resources = Resources::new();
        resources.insert::<Point2>(Point2 { x: 0.0, y: 0.0 });
        resources.insert::<String>("hello_world".to_string());

        assert_eq!(2, resources.data.len());
    }

    #[test]
    fn it_gets_resource() {
        let mut resources = Resources::new();
        resources.insert::<Point2>(Point2 { x: 0.0, y: 2.0 });
        resources.insert::<String>("world".to_string());

        assert_eq!(String::from("world"), *resources.get::<String>().unwrap());
    }
}
