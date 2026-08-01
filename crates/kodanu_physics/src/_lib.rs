mod components;
mod plugins;
mod resources;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::plugins::*;
    pub use crate::resources::*;
}

pub use crate::components::*;
pub use crate::plugins::*;
pub use crate::resources::*;

pub(crate) mod rapier3d;
