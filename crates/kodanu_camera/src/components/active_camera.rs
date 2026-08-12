#[derive(Default, Debug, Clone, Copy)]
pub struct ActiveCamera {
    is_active: bool,
}

impl ActiveCamera {
    pub fn new(is_active: bool) -> Self {
        Self { is_active }
    }
}

impl ActiveCamera {
    #[inline]
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
