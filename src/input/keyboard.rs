use crate::input::Layered;
use crate::node::WidgetId;

#[derive(Default, Debug)]
pub(crate) struct Keyboard {
    pub(crate) layered: Layered,
}

impl Keyboard {
    pub fn push_layer(&mut self, id: WidgetId) {
        self.layered.push_layer(id);
    }

    pub fn pop_layer(&mut self) {
        self.layered.pop_layer()
    }

    pub fn current_layer_root(&self) -> Option<WidgetId> {
        self.layered.current_root()
    }

    pub fn clear(&mut self) {
        self.layered.clear();
    }

    pub fn add(&mut self, id: WidgetId) {
        self.layered.insert(id, ());
    }

    pub fn remove(&mut self, removed: WidgetId) {
        self.layered.remove(removed)
    }
}
