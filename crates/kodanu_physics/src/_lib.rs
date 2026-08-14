mod components;
mod plugins;
mod resources;

pub mod prelude {
    pub use crate::{components::*, plugins::*, resources::*};
}

pub use crate::{components::*, plugins::*, resources::*};

pub(crate) mod rapier3d;
