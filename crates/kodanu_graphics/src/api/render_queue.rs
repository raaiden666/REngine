use crate::RenderItem;

#[derive(Default)]
pub struct RenderQueue {
    items: Vec<RenderItem>,
}

impl RenderQueue {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }
}

impl RenderQueue {
    #[inline]
    pub fn items(&self) -> &[RenderItem] {
        &self.items
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
    }

    #[inline]
    pub fn push(&mut self, item: RenderItem) {
        self.items.push(item);
    }
}
