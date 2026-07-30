use crate::BoundingSphere;

use kodanu_math::{Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vec3,
    distance: f32,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f32) -> Self {
        Self { normal, distance }
    }
}

impl Plane {
    #[inline]
    pub fn from_vec4(value: Vec4) -> Self {
        Self::new(value.truncate(), value.w)
    }

    #[inline]
    pub fn signed_distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.distance
    }

    #[inline]
    pub fn normilize(&mut self) {
        let length = self.normal.length();

        self.normal /= length;
        self.distance /= length;
    }

    #[inline]
    pub fn contains_sphere(&self, sphere: &BoundingSphere) -> bool {
        self.signed_distance(sphere.center()) >= -sphere.radius()
    }
}
