use std::collections::HashSet;

use slotmap::SecondaryMap;

use crate::geom::Rect;
use crate::node::{LayoutNode, WidgetId};

pub struct EventCtx<'a> {
    pub rect: Rect,
    pub current: WidgetId,
    pub children: &'a [WidgetId],
    pub(crate) computed: &'a SecondaryMap<WidgetId, LayoutNode>,
    pub(crate) hovered: &'a HashSet<WidgetId>,
    pub(crate) debug: &'a mut Vec<String>,
}

impl<'a> EventCtx<'a> {
    pub fn hovered(&self, id: WidgetId) -> bool {
        self.hovered.contains(&id)
    }

    pub fn get_rect(&self, id: WidgetId) -> Rect {
        self.computed[id].rect
    }

    pub fn debug(&mut self, debug: impl ToString) {
        self.debug.push(debug.to_string())
    }
}
