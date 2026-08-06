use crate::PluginRegistry;

pub trait Plugin: 'static {
    fn build(&self, app: &mut PluginRegistry);

    fn dependencies(&self, _app: &mut PluginRegistry) {}
}
