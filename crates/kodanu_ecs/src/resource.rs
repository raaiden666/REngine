use std::any::Any;

pub trait Resource: Any + Send + Sync {}

impl<T> Resource for T where T: Any + Send + Sync {}
