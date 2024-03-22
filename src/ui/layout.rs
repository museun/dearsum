use slotmap::{SecondaryMap, SlotMap};

use crate::{
    input::{Keyboard, Mouse},
    LayoutNode, Node, WidgetId,
};

pub struct Layout<'a> {
    pub nodes: &'a SlotMap<WidgetId, Node>,
    pub computed: &'a mut SecondaryMap<WidgetId, LayoutNode>,
    pub stack: &'a mut Vec<WidgetId>,
    pub mouse: &'a mut Mouse,
    pub keyboard: &'a mut Keyboard,
    pub clip_stack: &'a mut Vec<WidgetId>,
    pub debug: &'a mut Vec<String>,
}
