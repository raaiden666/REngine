use crate::AppBuilder;

pub trait Plugin {
    fn build(&self, app: &mut AppBuilder);
}
