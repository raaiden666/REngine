use crate::{Component, SparseSet, ViewStorage};

use std::{marker::PhantomData, ptr::NonNull};

pub struct WriteStorage<'w, T>
where
    T: Component,
{
    storage: NonNull<SparseSet<T>>,
    marker: PhantomData<&'w mut SparseSet<T>>,
}

impl<'w, T> WriteStorage<'w, T>
where
    T: Component,
{
    #[inline]
    pub(crate) fn new(storage: &'w mut SparseSet<T>) -> Self {
        Self {
            storage: NonNull::from(storage),
            marker: PhantomData,
        }
    }
}

impl<'w, T> ViewStorage<'w> for WriteStorage<'w, T>
where
    T: Component,
{
    type Item = &'w mut T;

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
        Some(unsafe { &mut *self.storage.as_mut().dense.as_mut_ptr().add(dense) })
    }

    #[inline]
    fn get_by_entity(&mut self, entity: u32) -> Self::Item {
        let dense = unsafe { self.storage.as_ref().dense_index(entity).unwrap_unchecked() };

        unsafe { &mut *self.storage.as_mut().dense.as_mut_ptr().add(dense) }
    }
}
