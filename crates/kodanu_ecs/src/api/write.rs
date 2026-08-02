use crate::{Component, Resource, ResourceAccess, ViewAccess, WorldCell, WriteStorage};

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Write<T>(PhantomData<T>);

impl<'w, T> ViewAccess<'w> for Write<T>
where
    T: Component,
{
    type Storage = WriteStorage<'w, T>;

    #[inline]
    fn fetch(world: WorldCell<'w>) -> Self::Storage {
        WriteStorage::new(world.storage_mut::<T>())
    }
}

impl<'w, T> ResourceAccess<'w> for Write<T>
where
    T: Resource,
{
    type Item = &'w mut T;

    #[inline]
    fn fetch(world: WorldCell<'w>) -> Self::Item {
        world.expect_resource_mut::<T>()
    }
}
