#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
}
