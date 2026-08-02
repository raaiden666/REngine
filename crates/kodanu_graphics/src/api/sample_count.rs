#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SampleCount {
    Single,
    Quad,
}

impl SampleCount {
    pub fn as_u32(self) -> u32 {
        match self {
            Self::Single => 1,
            Self::Quad => 4,
        }
    }
}
