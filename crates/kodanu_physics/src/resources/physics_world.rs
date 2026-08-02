#![allow(dead_code)]

use crate::{prelude::*, rapier3d::*};

use {kodanu_math::Vec3, kodanu_transform::Transform};

#[derive(Default)]
pub struct PhysicsWorld {
    physics: RapierPhysicsWorld,
}

impl PhysicsWorld {
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.physics.gravity = Vec3::new(gravity.x, gravity.y, gravity.z);
    }

    pub fn remove_rigid_body(&mut self, body: RigidBody) {
        if let Some(handle) = body.handle {
            self.physics.remove_body(handle);
        }
    }

    pub fn remove_collider(&mut self, collider: Collider) {
        if let Some(handle) = collider.handle {
            self.physics.remove_collider(handle);
        }
    }
}

impl PhysicsWorld {
    pub(crate) fn step(&mut self) {
        self.physics.step();
    }

    pub(crate) fn rigid_body(&self, handle: RapierRigidBodyHandle) -> Option<&RapierRigidBody> {
        self.physics.bodies.get(handle)
    }

    pub(crate) fn rigid_body_mut(
        &mut self,
        handle: RapierRigidBodyHandle,
    ) -> Option<&mut RapierRigidBody> {
        self.physics.bodies.get_mut(handle)
    }

    pub(crate) fn collider(&self, handle: RapierColliderHandle) -> Option<&RapierCollider> {
        self.physics.colliders.get(handle)
    }

    pub(crate) fn collider_mut(
        &mut self,
        handle: RapierColliderHandle,
    ) -> Option<&mut RapierCollider> {
        self.physics.colliders.get_mut(handle)
    }
}

impl PhysicsWorld {
    pub(crate) fn create_rigid_body(
        &mut self,
        rigid_body: &RigidBody,
        transform: &Transform,
    ) -> RapierRigidBodyHandle {
        self.physics
            .insert_body(rigid_body.builder(transform).build())
    }

    pub(crate) fn create_collider(
        &mut self,
        collider: &Collider,
        parent: &RigidBody,
    ) -> RapierColliderHandle {
        self.physics
            .insert_collider(collider.builder().build(), parent.handle)
    }
}
