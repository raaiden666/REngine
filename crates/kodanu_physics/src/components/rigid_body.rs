#![allow(dead_code)]

use crate::rapier3d::*;

use kodanu_transform::Transform;

pub struct RigidBody {
    pub(crate) handle: Option<RapierRigidBodyHandle>,
    pub(crate) body_type: RapierRigidBodyType,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::dynamic()
    }
}

impl RigidBody {
    pub(crate) fn new(body: RapierRigidBodyType) -> Self {
        Self {
            handle: None,
            body_type: body,
        }
    }
}

impl RigidBody {
    pub fn dynamic() -> Self {
        Self::new(RapierRigidBodyType::Dynamic)
    }

    pub fn fixed() -> Self {
        Self::new(RapierRigidBodyType::Fixed)
    }

    pub fn kinematic() -> Self {
        Self::new(RapierRigidBodyType::KinematicPositionBased)
    }
}

impl RigidBody {
    pub(crate) fn builder(&self, transform: &Transform) -> RapierRigidBodyBuilder {
        let (axis, angle) = transform.rotation().to_axis_angle();

        RapierRigidBodyBuilder::new(self.body_type)
            .translation(transform.position())
            .rotation(axis * angle)
    }
}

impl RigidBody {
    #[inline]
    pub fn handle(&self) -> &Option<RapierRigidBodyHandle> {
        &self.handle
    }

    #[inline]
    pub fn body(&self) -> &RapierRigidBodyType {
        &self.body_type
    }

    pub(crate) fn set_body(&mut self, body: RapierRigidBodyType) {
        self.body_type = body
    }
}
