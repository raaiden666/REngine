use crate::{Component, SparseSet, ViewStorage};

use std::{marker::PhantomData, ptr::NonNull};

pub struct ReadStorage<'w, T>
where
    T: Component,
{
    storage: NonNull<SparseSet<T>>,
    marker: PhantomData<&'w SparseSet<T>>,
}

impl<'w, T> ReadStorage<'w, T>
where
    T: Component,
{
    #[inline]
    pub(crate) fn new(storage: &'w SparseSet<T>) -> Self {
        Self {
            storage: NonNull::from(storage),
            marker: PhantomData,
        }
    }
}

impl<'w, T> ViewStorage<'w> for ReadStorage<'w, T>
where
    T: Component,
{
    type Item = &'w T;

    #[inline]
    fn len(&self) -> usize {
        unsafe { self.storage.as_ref().dense.len() }
    }

    #[inline]
    fn entity(&self, dense: usize) -> u32 {
        unsafe { *self.storage.as_ref().indices.get_unchecked(dense) }
    }

    #[inline]
    fn contains(&self, entity: u32) -> bool {
        unsafe { self.storage.as_ref().contains(entity) }
    }

    #[inline]
    fn get(&mut self, dense: usize) -> Option<Self::Item> {
        Some(unsafe { &*self.storage.as_ref().dense.as_ptr().add(dense) })
    }

    #[inline]
    fn get_by_entity(&mut self, entity: u32) -> Self::Item {
        let dense = unsafe { self.storage.as_ref().dense_index(entity).unwrap_unchecked() };

        unsafe { &*self.storage.as_ref().dense.as_ptr().add(dense) }
    }
}
