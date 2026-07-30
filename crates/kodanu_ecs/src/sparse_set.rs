use crate::{Component, ComponentStorage};

use std::{any::Any, mem::replace};

pub const INVALID_DENSE_INDEX: u32 = u32::MAX;

pub struct SparseSet<T> {
    pub(crate) sparse: Vec<u32>,
    pub(crate) indices: Vec<u32>,
    pub(crate) dense: Vec<T>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Vec::new(),
            indices: Vec::new(),
            dense: Vec::new(),
        }
    }
}

impl<T> SparseSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sparse: Vec::with_capacity(capacity),
            indices: Vec::with_capacity(capacity),
            dense: Vec::with_capacity(capacity),
        }
    }
}

impl<T> SparseSet<T> {
    #[inline]
    pub fn contains(&self, entity: u32) -> bool {
        self.dense_index(entity).is_some()
    }

    #[inline]
    pub fn get(&self, entity: u32) -> Option<&T> {
        let dense = self.dense_index(entity)?;
        Some(&self.dense[dense])
    }

    #[inline]
    pub fn get_mut(&mut self, entity: u32) -> Option<&mut T> {
        let dense = self.dense_index(entity)?;
        Some(&mut self.dense[dense])
    }

    #[inline]
    pub fn dense_mut(&mut self) -> &mut Vec<T> {
        &mut self.dense
    }

    #[inline]
    pub fn insert(&mut self, entity: u32, component: T) -> Option<T> {
        self.ensure_capacity(entity);

        if let Some(dense) = self.dense_index(entity) {
            return Some(replace(&mut self.dense[dense], component));
        }

        let dense = self.dense.len() as u32;

        self.sparse[entity as usize] = dense;

        self.indices.push(entity);
        self.dense.push(component);

        None
    }

    #[inline]
    pub fn remove(&mut self, entity: u32) -> Option<T> {
        let dense = self.dense_index(entity)?;
        self.sparse[entity as usize] = INVALID_DENSE_INDEX;

        let component = self.dense.swap_remove(dense);
        self.indices.swap_remove(dense);

        if dense < self.indices.len() {
            let moved = self.indices[dense];
            self.sparse[moved as usize] = dense as u32;
        }

        Some(component)
    }
}

impl<T> SparseSet<T> {
    #[inline]
    pub(crate) fn dense_index(&self, entity: u32) -> Option<usize> {
        let dense = *self.sparse.get(entity as usize)?;

        if dense == INVALID_DENSE_INDEX {
            return None;
        }

        Some(dense as usize)
    }

    #[inline]
    fn ensure_capacity(&mut self, entity: u32) {
        let requied = entity as usize + 1;

        if self.sparse.len() < requied {
            self.sparse.resize(requied, INVALID_DENSE_INDEX);
        }
    }
}

impl<T: Component> ComponentStorage for SparseSet<T> {
    #[inline]
    fn remove_from_entity(&mut self, entity: u32) {
        let _ = self.remove(entity);
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
