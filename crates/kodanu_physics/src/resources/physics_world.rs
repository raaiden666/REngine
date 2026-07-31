#![allow(dead_code)]

use crate::{
    prelude::{Collider, RigidBody},
    rapier3d::*,
};

use kodanu_transform::Transform;

#[derive(Default)]
pub struct PhysicsWorld {
    physics: RapierPhysicsWorld,
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
            .bodies
            .insert(rigid_body.builder(transform).build())
    }

    pub(crate) fn create_collider(
        &mut self,
        collider: &Collider,
        parent: RapierRigidBodyHandle,
    ) -> RapierColliderHandle {
        self.physics.colliders.insert_with_parent(
            collider.builder().build(),
            parent,
            &mut self.physics.bodies,
        )
    }
}
