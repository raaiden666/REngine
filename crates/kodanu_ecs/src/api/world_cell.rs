#![allow(dead_code)]

use crate::{
    Bundle, Component, Entity, Resource, ResourceAccess, SparseSet, View, ViewAccess, World,
};

use std::{marker::PhantomData, ptr::NonNull};

#[derive(Clone, Copy)]
pub struct WorldCell<'w> {
    world: NonNull<World>,
    marker: PhantomData<&'w mut World>,
}

impl<'w> WorldCell<'w> {
    pub fn from_world_mut(world: &'w mut World) -> Self {
        Self {
            world: NonNull::from(world),
            marker: PhantomData,
        }
    }
}

impl<'w> WorldCell<'w> {
    #[inline]
    pub fn spawn<C: Component>(self, component: C) -> Entity {
        unsafe { (*self.world.as_ptr()).spawn(component) }
    }

    #[inline]
    pub fn spawn_bundle<B: Bundle>(self, bundle: B) -> Entity {
        unsafe { (*self.world.as_ptr()).spawn_bundle(bundle) }
    }

    #[inline]
    pub fn despawn(self, entity: Entity) -> bool {
        unsafe { (*self.world.as_ptr()).despawn(entity) }
    }

    #[inline]
    pub fn insert_component<C: Component>(self, entity: Entity, component: C) -> Option<C> {
        unsafe {
            (*self.world.as_ptr())
                .storage_mut::<C>()
                .insert(entity.id, component)
        }
    }

    #[inline]
    pub fn component<C: Component>(self, entity: Entity) -> Option<&'w C> {
        unsafe { (*self.world.as_ptr()).component::<C>(entity) }
    }

    #[inline]
    pub fn component_mut<C: Component>(self, entity: Entity) -> Option<&'w mut C> {
        unsafe { (*self.world.as_ptr()).component_mut::<C>(entity) }
    }

    #[inline]
    pub fn storage<C: Component>(self) -> &'w SparseSet<C> {
        unsafe { (&mut *self.world.as_ptr()).storage::<C>() }
    }

    #[inline]
    pub fn storage_mut<C: Component>(self) -> &'w mut SparseSet<C> {
        unsafe { (&mut *self.world.as_ptr()).storage_mut::<C>() }
    }
}

impl<'w> WorldCell<'w> {
    #[inline]
    pub fn insert_resource<R: Resource>(self, resource: R) -> Option<R> {
        unsafe { (&mut *self.world.as_ptr()).insert_resource(resource) }
    }

    #[inline]
    pub fn resource<R: Resource>(self) -> Option<&'w R> {
        unsafe { (*self.world.as_ptr()).resource::<R>() }
    }

    #[inline]
    pub fn resource_mut<R: Resource>(self) -> Option<&'w mut R> {
        unsafe { (*self.world.as_ptr()).resource_mut::<R>() }
    }

    #[inline]
    pub fn expect_resource<R: Resource>(self) -> &'w R {
        unsafe { (*self.world.as_ptr()).expect_resource::<R>() }
    }

    #[inline]
    pub fn expect_resource_mut<R: Resource>(self) -> &'w mut R {
        unsafe { (*self.world.as_ptr()).expect_resource_mut::<R>() }
    }

    #[inline]
    pub fn contains_resource<R: Resource>(self) -> bool {
        unsafe { (*self.world.as_ptr()).contains_resource::<R>() }
    }
}

impl<'w> WorldCell<'w> {
    #[inline]
    pub fn view<A>(self) -> View<'w, A::Storage>
    where
        A: ViewAccess<'w>,
    {
        View::new(A::fetch(self))
    }

    #[inline]
    pub fn res<A>(self) -> A::Item
    where
        A: ResourceAccess<'w>,
    {
        A::fetch(self)
    }
}
