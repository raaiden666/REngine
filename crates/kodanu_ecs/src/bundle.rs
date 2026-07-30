use crate::{Component, Entity, World};

pub trait Bundle {
    fn insert(self, entity: Entity, world: &mut World);
}

impl<A, B> Bundle for (A, B)
where
    A: Component,
    B: Component,
{
    #[inline]
    fn insert(self, entity: Entity, world: &mut World) {
        let (a, b) = self;

        world.insert_component(entity, a);
        world.insert_component(entity, b);
    }
}

impl<A, B, C> Bundle for (A, B, C)
where
    A: Component,
    B: Component,
    C: Component,
{
    #[inline]
    fn insert(self, entity: Entity, world: &mut World) {
        let (a, b, c) = self;

        world.insert_component(entity, a);
        world.insert_component(entity, b);
        world.insert_component(entity, c);
    }
}
