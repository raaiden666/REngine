#![allow(dead_code)]

use crate::Entity;

#[derive(Default)]
pub struct EntityAllocator {
    free: Vec<u32>,
    gens: Vec<u32>,
}

impl EntityAllocator {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            free: Vec::with_capacity(capacity),
            gens: Vec::with_capacity(capacity),
        }
    }
}

impl EntityAllocator {
    #[inline]
    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.free.pop() {
            let gens = self.gens[id as usize];

            Entity { id, gens }
        } else {
            let id = self.gens.len() as u32;

            self.gens.push(0);

            Entity { id, gens: 0 }
        }
    }

    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.gens[entity.id as usize] += 1;

        self.free.push(entity.id);

        true
    }

    #[inline]
    pub fn is_alive(&self, entity: Entity) -> bool {
        let Some(&gens) = self.gens.get(entity.id as usize) else {
            return false;
        };

        gens == entity.gens
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.gens.len() - self.free.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.free.clear();
        self.gens.clear();
    }
}
