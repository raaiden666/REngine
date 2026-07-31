#![allow(dead_code)]

use crate::rapier3d::*;

use kodanu_math::Vec3;

pub struct Collider {
    pub(crate) handle: Option<RapierColliderHandle>,
    pub(crate) shape: RapierColliderShape,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            handle: None,
            shape: RapierColliderShape::cuboid(0.5, 0.5, 0.5),
        }
    }
}

impl Collider {
    pub fn cube(size: Vec3) -> Self {
        let half_size = size * 0.5;

        Self {
            handle: None,
            shape: RapierColliderShape::cuboid(half_size.x, half_size.y, half_size.z),
        }
    }

    pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            handle: None,
            shape: RapierColliderShape::triangle(a, b, c),
        }
    }

    pub fn sphere(radius: f32) -> Self {
        let radius = radius * 0.5;
        Self {
            handle: None,
            shape: RapierColliderShape::ball(radius),
        }
    }
}

impl Collider {
    pub(crate) fn builder(&self) -> RapierColliderBuilder {
        RapierColliderBuilder::new(self.shape.clone())
    }
}

impl Collider {
    #[inline]
    pub fn handle(&self) -> &Option<RapierColliderHandle> {
        &self.handle
    }

    #[inline]
    pub fn shape(&self) -> &RapierColliderShape {
        &self.shape
    }

    pub(crate) fn set_handle(&mut self, handle: RapierColliderHandle) {
        self.handle = Some(handle)
    }
}
