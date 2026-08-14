use kodanu_color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material {
    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }
}
