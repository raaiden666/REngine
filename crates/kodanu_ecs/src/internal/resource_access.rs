use crate::WorldCell;

pub trait ResourceAccess<'w> {
    type Item;

    fn fetch(world: WorldCell<'w>) -> Self::Item;
}
