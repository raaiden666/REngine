#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Entity {
    pub(crate) id: u32,
    pub(crate) gens: u32,
}

impl Entity {
    #[inline]
    pub const fn new(id: u32, gens: u32) -> Self {
        Self { id, gens }
    }
}

impl Entity {
    #[inline]
    pub const fn id(self) -> u32 {
        self.id
    }

    #[inline]
    pub const fn gens(self) -> u32 {
        self.gens
    }
}
