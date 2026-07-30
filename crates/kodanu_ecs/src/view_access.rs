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
