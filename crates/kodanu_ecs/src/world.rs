use crate::{
    Bundle, Component, ComponentRegistry, Entity, EntityAllocator, Resource, ResourceAccess,
    ResourceRegistry, SparseSet, View, ViewAccess, WorldCell,
};

#[derive(Default)]
pub struct World {
    pub(crate) allocator: EntityAllocator,
    pub(crate) storage: ComponentRegistry,
    pub(crate) resources: ResourceRegistry,
}

impl World {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            allocator: EntityAllocator::with_capacity(capacity),
            storage: ComponentRegistry::with_capacity(capacity),
            resources: ResourceRegistry::with_capacity(capacity),
        }
    }
}

impl World {
    #[inline]
    pub fn spawn<T: Component>(&mut self, component: T) -> Entity {
        let entity = self.allocator.create();
        self.insert_component(entity, component);
        entity
    }

    #[inline]
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        let entity = self.allocator.create();
        bundle.insert(entity, self);
        entity
    }

    #[inline]
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.allocator.destroy(entity) {
            return false;
        }

        self.storage.remove_from_entity(entity.id);

        true
    }

    #[inline]
    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) -> Option<T> {
        self.storage.insert(entity.id, component)
    }

    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> Option<T> {
        self.storage.remove(entity.id)
    }

    #[inline]
    pub fn component<T: Component>(&mut self, entity: Entity) -> Option<&T> {
        if !self.allocator.is_alive(entity) {
            return None;
        }

        self.storage.get(entity.id)
    }

    #[inline]
    pub fn component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.allocator.is_alive(entity) {
            return None;
        }

        self.storage.get_mut(entity.id)
    }

    #[inline]
    pub(crate) fn storage<T: Component>(&mut self) -> &SparseSet<T> {
        self.storage.storage::<T>()
    }

    #[inline]
    pub(crate) fn storage_mut<T: Component>(&mut self) -> &mut SparseSet<T> {
        self.storage.storage_mut::<T>()
    }

    #[inline]
    pub fn cell(&mut self) -> WorldCell<'_> {
        WorldCell::from_world_mut(self)
    }
}

impl World {
    #[inline]
    pub fn insert_resource<T: Resource>(&mut self, resource: T) -> Option<T> {
        self.resources.insert(resource)
    }

    #[inline]
    pub fn remove_resource<T: Resource>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }

    #[inline]
    pub fn resource<T: Resource>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    #[inline]
    pub fn resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    #[inline]
    pub fn expect_resource<T: Resource>(&self) -> &T {
        self.resources
            .get::<T>()
            .expect("Resource is not registered")
    }

    #[inline]
    pub fn expect_resource_mut<T: Resource>(&mut self) -> &mut T {
        self.resources
            .get_mut::<T>()
            .expect("Resource is not registered")
    }

    #[inline]
    pub fn contains_resource<T: Resource>(&self) -> bool {
        self.resources.contains::<T>()
    }
}

impl World {
    #[inline]
    pub fn view<'w, A>(&'w mut self) -> View<'w, A::Storage>
    where
        A: ViewAccess<'w>,
    {
        View::new(A::fetch(WorldCell::from_world_mut(self)))
    }

    #[inline]
    pub fn res<'w, A>(&'w mut self) -> A::Item
    where
        A: ResourceAccess<'w>,
    {
        A::fetch(WorldCell::from_world_mut(self))
    }
}
