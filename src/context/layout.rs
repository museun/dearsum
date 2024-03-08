use crate::{
    geom::{Constraints, Pos2, Rect, Size, Vec2},
    ui::Layout,
    LayoutNode, Node, WidgetId,
};

// TODO redo this api
pub struct LayoutCtx<'a: 'b, 'b> {
    pub current: WidgetId,
    pub children: &'a [WidgetId],
    pub(crate) layout: &'b mut Layout<'a>,
}

impl<'a: 'b, 'b> LayoutCtx<'a, 'b> {
    pub fn compute(&mut self, child: WidgetId, input: Constraints) -> Size {
        let Some(node) = self.layout.nodes.get(child) else {
            return Size::ZERO;
        };

        self.layout.stack.push(child);

        let widget = &node.widget;
        let size = widget.layout(
            LayoutCtx {
                current: child,
                children: node.children(),
                layout: self.layout,
            },
            input,
        );

        let new_layer_mouse = self.layout.mouse.current_layer_root() == Some(child);
        let new_layer_keyboard = self.layout.keyboard.current_layer_root() == Some(child);

        let interest = node.widget.interest();

        if interest.is_mouse_any() {
            self.layout.mouse.add(child, interest);
        }
        if interest.is_key_input() {
            self.layout.keyboard.add(child);
        }

        if new_layer_mouse {
            self.layout.mouse.pop_layer();
        }

        if new_layer_keyboard {
            self.layout.keyboard.pop_layer();
        }

        let clipping = self.layout.clip_stack.last() == Some(&child); // this is wrong

        let clipped_by = if clipping {
            self.layout.clip_stack.iter().nth_back(2).copied()
        } else {
            self.layout.clip_stack.last().copied()
        };

        if clipping && clipped_by.is_none() {
            self.layout.clip_stack.pop();
        }

        self.layout.computed.insert(
            child,
            LayoutNode {
                rect: Rect::from_min_size(Pos2::ZERO, size.into()),
                interest: widget.interest(),
                clipped_by,
                clipping,
            },
        );

        assert_eq!(self.layout.stack.pop(), Some(child));

        size
    }

    pub fn get_node(&self, id: WidgetId) -> &Node {
        &self.layout.nodes[id]
    }

    pub fn get_layout(&self, id: WidgetId) -> &LayoutNode {
        &self.layout.computed[id]
    }

    pub fn set_pos(&mut self, child: WidgetId, pos: Pos2) {
        if let Some(node) = self.layout.computed.get_mut(child) {
            node.rect += pos;
            // node.rect = Rect::from_min_size(pos, node.rect.size());
        }
    }

    pub fn set_size(&mut self, child: WidgetId, size: Vec2) {
        if let Some(node) = self.layout.computed.get_mut(child) {
            node.rect += size;
        }
    }

    pub fn get_rect(&self, child: WidgetId) -> Option<Rect> {
        self.layout.computed.get(child).map(|n| n.rect)
    }

    pub fn new_layer_for(&mut self, id: WidgetId) {
        self.layout.mouse.push_layer(id);
        self.layout.keyboard.push_layer(id);
    }

    pub fn enable_clipping_for(&mut self, id: WidgetId) {
        self.layout.clip_stack.push(id)
    }

    pub fn enable_clipping(&mut self) {
        self.enable_clipping_for(self.current)
    }

    pub fn new_layer(&mut self) {
        self.new_layer_for(self.current)
    }
}
