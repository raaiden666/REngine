pub trait ViewStorage<'w>: Sized {
    type Item;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn entity(&self, dense: usize) -> u32;

    fn contains(&self, entity: u32) -> bool;

    fn get(&mut self, dense: usize) -> Option<Self::Item>;

    fn get_by_entity(&mut self, entity: u32) -> Self::Item;
}

impl<'w, A, B> ViewStorage<'w> for (A, B)
where
    A: ViewStorage<'w>,
    B: ViewStorage<'w>,
{
    type Item = (A::Item, B::Item);

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn entity(&self, dense: usize) -> u32 {
        self.0.entity(dense)
    }

    #[inline]
    fn contains(&self, entity: u32) -> bool {
        self.0.contains(entity) && self.1.contains(entity)
    }

    #[inline]
    fn get(&mut self, dense: usize) -> Option<Self::Item> {
        let entity = self.0.entity(dense);

        if !self.1.contains(entity) {
            return None;
        }

        unsafe {
            Some((
                self.0.get(dense).unwrap_unchecked(),
                self.1.get_by_entity(entity),
            ))
        }
    }

    #[inline]
    fn get_by_entity(&mut self, entity: u32) -> Self::Item {
        (self.0.get_by_entity(entity), self.1.get_by_entity(entity))
    }
}
impl<'w, A, B, C> ViewStorage<'w> for (A, B, C)
where
    A: ViewStorage<'w>,
    B: ViewStorage<'w>,
    C: ViewStorage<'w>,
{
    type Item = (A::Item, B::Item, C::Item);

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn entity(&self, dense: usize) -> u32 {
        self.0.entity(dense)
    }

    #[inline]
    fn contains(&self, entity: u32) -> bool {
        self.0.contains(entity) && self.1.contains(entity) && self.2.contains(entity)
    }

    #[inline]
    fn get(&mut self, dense: usize) -> Option<Self::Item> {
        let entity = self.0.entity(dense);

        if !self.1.contains(entity) {
            return None;
        }

        unsafe {
            Some((
                self.0.get(dense).unwrap_unchecked(),
                self.1.get_by_entity(entity),
                self.2.get_by_entity(entity),
            ))
        }
    }

    #[inline]
    fn get_by_entity(&mut self, entity: u32) -> Self::Item {
        (
            self.0.get_by_entity(entity),
            self.1.get_by_entity(entity),
            self.2.get_by_entity(entity),
        )
    }
}
