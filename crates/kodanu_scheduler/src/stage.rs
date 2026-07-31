#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Stage {
    Startup,
    PreFixedUpdate,
    FixedUpdate,
    PostFixedUpdate,
    PreUpdate,
    Update,
    LateUpdate,
    EndFrame,
    Render,
}

impl Stage {
    pub const COUNT: usize = 9;

    #[inline]
    pub fn as_usize(self) -> usize {
        self as usize
    }
}
