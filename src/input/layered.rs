use crate::debug_fmt;
use crate::node::WidgetId;

#[derive(Debug)]
pub struct Layered<T = ()> {
    pub layers: Vec<Vec<Item<T>>>,
    pub stack: Vec<Item<usize>>,
}

pub struct Item<T> {
    pub id: WidgetId,
    pub item: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Item<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item")
            .field("id", &debug_fmt::id(self.id))
            .field("item", &self.item)
            .finish()
    }
}

impl<T> Default for Layered<T> {
    fn default() -> Self {
        Self {
            layers: Vec::new(),
            stack: Vec::new(),
        }
    }
}

impl<T> Layered<T> {
    pub fn clear(&mut self) {
        std::mem::take(self);
    }

    pub fn insert(&mut self, id: WidgetId, item: T) {
        self.stack
            .last()
            .and_then(|Item { item, .. }| self.layers.get_mut(*item))
            .unwrap()
            .push(Item { id, item })
    }

    pub fn remove(&mut self, removed: WidgetId) {
        self.stack.retain(|Item { id, .. }| *id != removed);
        for layer in &mut self.layers {
            layer.retain(|Item { id, .. }| *id != removed);
        }
    }

    pub fn current_root(&self) -> Option<WidgetId> {
        self.stack.last().map(|&Item { id, .. }| id)
    }

    pub fn push_layer(&mut self, id: WidgetId) {
        let item = self.layers.len();
        self.layers.push(vec![]);
        self.stack.push(Item { id, item })
    }

    pub fn pop_layer(&mut self) {
        debug_assert!(
            self.stack.pop().is_some(),
            "cannot pop a layer without one existing"
        )
    }

    // this is slow
    pub fn iter(&self) -> impl Iterator<Item = (&WidgetId, &T)> + '_ {
        self.layers
            .iter()
            .rev()
            .flatten()
            .map(|item| (&item.id, &item.item))
    }
}
