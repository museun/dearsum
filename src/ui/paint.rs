use crate::context::PaintCtx;
use crate::geom::{pos2, Rect};
use crate::node::WidgetId;
use crate::paint::{CroppedSurface as Canvas, Styled};

#[derive(Default, Debug)]
pub struct Paint {
    clip_stack: Vec<Rect>,
    debug: Vec<String>,
}

impl Paint {
    pub fn paint_all(&mut self, ui: &super::Inner, canvas: &mut Canvas<'_>) {
        self.paint(ui, canvas, ui.root());
        self.paint_debug(canvas)
    }

    pub fn debug(&mut self, label: impl ToString) {
        self.debug
            .extend(label.to_string().lines().map(|s| s.to_string()))
    }

    // TODO paint this at the right-top instead (most interesting things are the left-top)
    fn paint_debug(&mut self, canvas: &mut Canvas<'_>) {
        let mut pos = canvas.rect().right_top();
        for debug in self.debug.drain(..) {
            let styled = Styled::new(debug).fg(0xFF0000).bg(0x000000);
            let size = styled.size();
            canvas
                .crop(Rect::from_min_size(pos - pos2(size.x, 0), size))
                .draw(styled);
            pos.y += size.y;
        }
    }

    pub(crate) fn paint(&mut self, ui: &super::Inner, canvas: &mut Canvas<'_>, id: WidgetId) {
        let computed = ui.computed.borrow();
        let Some(layout) = computed.get(id) else {
            return;
        };

        if layout.clipping {
            self.push_clip(layout.rect);
        }

        let mut rect = layout.rect;

        if let Some(parent) = layout.clipped_by {
            rect = computed[parent].rect.intersection(rect);
        }

        ui.stack.borrow_mut().push(id);

        let node = &ui.nodes.borrow()[id];

        node.widget.paint(PaintCtx {
            rect,
            ui,
            current_id: id,
            children: &node.children,
            canvas: &mut canvas.crop(rect),
            paint: self,
        });

        assert_eq!(Some(id), ui.stack.borrow_mut().pop());

        if layout.clipping {
            self.pop_clip();
        }
    }

    fn push_clip(&mut self, mut rect: Rect) {
        if let Some(previous) = self.clip_stack.last() {
            rect = rect.intersection(*previous);
        }
        self.clip_stack.push(rect);
    }

    fn pop_clip(&mut self) {
        debug_assert!(
            self.clip_stack.pop().is_some(),
            "cannot pop a paint clip without pushing one first"
        )
    }
}
