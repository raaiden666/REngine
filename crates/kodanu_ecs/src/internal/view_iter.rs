use crate::ViewStorage;

use std::{iter::FusedIterator, marker::PhantomData};

pub struct ViewIter<'w, S>
where
    S: ViewStorage<'w>,
{
    storage: S,
    dense: usize,
    marker: PhantomData<&'w ()>,
}

impl<'w, S> ViewIter<'w, S>
where
    S: ViewStorage<'w>,
{
    #[inline]
    pub(crate) fn new(storage: S) -> Self {
        Self {
            storage,
            dense: 0,
            marker: PhantomData,
        }
    }
}

impl<'w, S> Iterator for ViewIter<'w, S>
where
    S: ViewStorage<'w>,
{
    type Item = S::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.dense < self.storage.len() {
            let dense = self.dense;
            self.dense += 1;

            if let Some(item) = self.storage.get(dense) {
                return Some(item);
            }
        }

        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.storage.len() - self.dense))
    }
}

impl<'w, S> FusedIterator for ViewIter<'w, S> where S: ViewStorage<'w> {}
