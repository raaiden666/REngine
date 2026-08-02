use crate::{ViewStorage, WorldCell};

pub trait ViewAccess<'w> {
    type Storage: ViewStorage<'w>;

    fn fetch(world: WorldCell<'w>) -> Self::Storage;
}

impl<'w, A, B> ViewAccess<'w> for (A, B)
where
    A: ViewAccess<'w>,
    B: ViewAccess<'w>,
{
    type Storage = (A::Storage, B::Storage);

    #[inline]
    fn fetch(world: WorldCell<'w>) -> Self::Storage {
        (A::fetch(world), B::fetch(world))
    }
}
impl<'w, A, B, C> ViewAccess<'w> for (A, B, C)
where
    A: ViewAccess<'w>,
    B: ViewAccess<'w>,
    C: ViewAccess<'w>,
{
    type Storage = (A::Storage, B::Storage, C::Storage);

    #[inline]
    fn fetch(world: WorldCell<'w>) -> Self::Storage {
        (A::fetch(world), B::fetch(world), C::fetch(world))
    }
}
