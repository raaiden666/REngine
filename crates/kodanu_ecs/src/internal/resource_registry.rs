#![allow(dead_code)]

use crate::Resource;

use {
    hashbrown::HashMap,
    std::any::{Any, TypeId},
};

#[derive(Default)]
pub struct ResourceRegistry {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceRegistry {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            resources: HashMap::with_capacity(capacity),
        }
    }
}

impl ResourceRegistry {
    #[inline]
    pub fn contains<T: Resource>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    pub fn insert<T: Resource>(&mut self, resource: T) -> Option<T> {
        let previous = self.resources.insert(TypeId::of::<T>(), Box::new(resource));

        previous
            .and_then(|resource| resource.downcast::<T>().ok())
            .map(|resource| *resource)
    }

    #[inline]
    pub fn remove<T: Resource>(&mut self) -> Option<T> {
        self.resources
            .remove(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast::<T>().ok().map(|resource| *resource))
    }

    #[inline]
    pub fn get<T: Resource>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_ref::<T>())
    }

    #[inline]
    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_mut::<T>())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.resources.clear();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}
