use std::collections::{HashMap, HashSet};

use crate::input::{Interest, Layered};
use crate::terminal::event::MouseButton;
use crate::{geom::Pos2, node::WidgetId};

#[derive(Debug, Default)]
pub(crate) struct Mouse {
    pub(crate) pos: Pos2,
    pub(crate) prev: Pos2,
    pub(crate) layered: Layered<Interest>,
    pub(crate) mouse_over: HashSet<WidgetId>,
    pub(crate) buttons: HashMap<MouseButton, ButtonState>,
}

impl Mouse {
    pub fn push_layer(&mut self, id: WidgetId) {
        self.layered.push_layer(id);
    }

    pub fn hovered(&mut self, id: WidgetId) {
        self.mouse_over.insert(id);
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

    pub fn add(&mut self, id: WidgetId, interest: Interest) {
        self.layered.insert(id, interest);
    }

    pub fn remove(&mut self, removed: WidgetId) {
        self.layered.remove(removed)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum ButtonState {
    Held,
    Released,
}
