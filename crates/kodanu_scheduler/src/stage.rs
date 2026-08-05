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
    PreRender,
    Render,
    PostRender,
}

impl Stage {
    pub const COUNT: usize = 12;

    #[inline]
    pub fn as_usize(self) -> usize {
        self as usize
    }
}
