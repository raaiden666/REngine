use std::any::Any;

pub trait Component: Any + Send + Sync {}

impl<T> Component for T where T: Any + Send + Sync {}
