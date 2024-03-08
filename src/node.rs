slotmap::new_key_type! {
    pub struct WidgetId;
}

use crate::{
    debug_fmt,
    geom::{FlexFit, Flow, Rect},
    input::Interest,
    widget::ErasedWidget,
};

pub struct Node {
    pub(crate) widget: Box<dyn ErasedWidget>,
    pub(crate) parent: Option<WidgetId>,
    pub(crate) children: Vec<WidgetId>,
    pub(crate) next: usize,
}

impl Node {
    pub fn flex(&self) -> (u16, FlexFit) {
        self.widget.flex()
    }

    pub fn flow(&self) -> Flow {
        self.widget.flow()
    }

    pub fn children(&self) -> &[WidgetId] {
        &self.children
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("widget", &self.widget)
            .field("parent", &self.parent.map(debug_fmt::id))
            .field("children", &debug_fmt::vec(&self.children))
            .field("next", &self.next)
            .finish()
    }
}

pub struct LayoutNode {
    pub rect: Rect,
    pub(crate) interest: Interest,
    pub(crate) clipping: bool,
    pub(crate) clipped_by: Option<WidgetId>,
}

impl std::fmt::Debug for LayoutNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutNode")
            .field("rect", &crate::debug_fmt::rect(self.rect))
            .field("interest", &self.interest)
            .field("clipping", &self.clipping)
            .field("clipped_by", &self.clipped_by)
            .finish()
    }
}
