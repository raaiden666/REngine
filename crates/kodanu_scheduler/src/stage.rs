#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Stage {
    Startup,
    PreUpdate,
    Update,
    LateUpdate,
    EndFrame,
    Render,
}

impl Stage {
    pub const COUNT: usize = 6;

    #[inline]
    pub fn as_usize(self) -> usize {
        self as usize
    }
}
