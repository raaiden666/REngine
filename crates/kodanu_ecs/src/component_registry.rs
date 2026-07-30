use crate::{Component, ComponentStorage, SparseSet};

use {hashbrown::HashMap, std::any::TypeId};

#[derive(Default)]
pub struct ComponentRegistry {
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl ComponentRegistry {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storages: HashMap::with_capacity(capacity),
        }
    }
}

impl ComponentRegistry {
    #[inline]
    pub fn insert<T: Component>(&mut self, entity: u32, component: T) -> Option<T> {
        self.storage_mut::<T>().insert(entity, component)
    }

    #[inline]
    pub fn remove<T: Component>(&mut self, entity: u32) -> Option<T> {
        self.storage_mut::<T>().remove(entity)
    }

    #[inline]
    pub fn remove_from_entity(&mut self, entity: u32) {
        for storage in self.storages.values_mut() {
            storage.remove_from_entity(entity);
        }
    }

    #[inline]
    pub fn get<T: Component>(&mut self, entity: u32) -> Option<&T> {
        self.storage::<T>().get(entity)
    }

    #[inline]
    pub fn get_mut<T: Component>(&mut self, entity: u32) -> Option<&mut T> {
        self.storage_mut::<T>().get_mut(entity)
    }

    #[inline]
    pub fn contains<T: Component>(&mut self, entity: u32) -> bool {
        self.storage::<T>().contains(entity)
    }

    #[inline]
    pub fn contains_storage<T: Component>(&self) -> bool {
        self.storages.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    pub fn storage<T: Component>(&mut self) -> &SparseSet<T> {
        let id = TypeId::of::<T>();

        self.storages
            .entry(id)
            .or_insert_with(|| Box::new(SparseSet::<T>::default()));

        self.storages
            .get_mut(&id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<SparseSet<T>>()
            .expect("Storage type mismatch")
    }

    #[inline]
    pub fn storage_mut<T: Component>(&mut self) -> &mut SparseSet<T> {
        let id = TypeId::of::<T>();

        self.storages
            .entry(id)
            .or_insert_with(|| Box::new(SparseSet::<T>::default()));

        self.storages
            .get_mut(&id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<SparseSet<T>>()
            .expect("Storage type mismatch")
    }
}
