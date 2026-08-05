use kodanu_math::Mat4;

#[derive(Default, Debug)]
pub struct EditorView {
    view_projection: Mat4,
}

impl EditorView {
    #[inline]
    pub fn view_projection(&self) -> Mat4 {
        self.view_projection
    }

    #[inline]
    pub fn set_view_projection(&mut self, view_projection: Mat4) {
        self.view_projection = view_projection;
    }
}
