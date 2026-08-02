use crate::{Component, ReadStorage, Resource, ResourceAccess, ViewAccess, WorldCell};

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Read<T>(PhantomData<T>);

impl<'w, T> ViewAccess<'w> for Read<T>
where
    T: Component,
{
    type Storage = ReadStorage<'w, T>;

    #[inline]
    fn fetch(world: WorldCell<'w>) -> Self::Storage {
        ReadStorage::new(world.storage::<T>())
    }
}

impl<'w, T> ResourceAccess<'w> for Read<T>
where
    T: Resource,
{
    type Item = &'w T;

    #[inline]
    fn fetch(world: WorldCell<'w>) -> Self::Item {
        world.expect_resource::<T>()
    }
}
