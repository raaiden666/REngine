use crate::{ViewIter, ViewStorage};

use std::marker::PhantomData;

pub struct View<'w, S>
where
    S: ViewStorage<'w>,
{
    storage: S,
    marker: PhantomData<&'w ()>,
}

impl<'w, S> View<'w, S>
where
    S: ViewStorage<'w>,
{
    #[inline]
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            marker: PhantomData,
        }
    }
}

impl<'w, S> View<'w, S>
where
    S: ViewStorage<'w>,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'w, S> IntoIterator for View<'w, S>
where
    S: ViewStorage<'w>,
{
    type Item = S::Item;
    type IntoIter = ViewIter<'w, S>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ViewIter::new(self.storage)
    }
}
