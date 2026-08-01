use crate::PluginRegistry;

pub trait Plugin {
    fn build(&self, app: &mut PluginRegistry);
}
