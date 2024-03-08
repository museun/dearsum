use std::{cell::Ref, hash::Hash, time::Duration};

use crate::geom::{vec2, Pos2, Rect};
use crate::node::{LayoutNode, WidgetId};
use crate::paint::{shape::Shape, Cell, CroppedSurface as Canvas};
use crate::ui::{Inner, Paint};

pub struct PaintCtx<'a: 'c, 'c> {
    pub rect: Rect,
    pub current_id: WidgetId,
    pub children: &'a [WidgetId],

    pub(crate) canvas: &'a mut Canvas<'c>,
    pub(crate) ui: &'a Inner,
    pub(crate) paint: &'a mut Paint,
}

impl<'a: 'c, 'c> PaintCtx<'a, 'c> {
    pub fn paint(&mut self, id: WidgetId) {
        self.paint.paint(self.ui, self.canvas, id);
    }

    pub fn draw(&mut self, shape: impl Shape) {
        self.canvas.draw(shape)
    }

    pub fn crop(&mut self, rect: Rect) -> Canvas<'_> {
        self.canvas.crop(rect)
    }

    pub fn draw_cropped(&mut self, rect: Rect, shape: impl Shape) {
        self.canvas.crop(rect).draw(shape)
    }

    // TODO this could just be a call that does crop + draw shape
    pub fn put(&mut self, pos: Pos2, cell: Cell) {
        self.canvas
            .crop(Rect::from_min_size(pos, vec2(1, 0)))
            .draw(cell)
    }

    pub fn get_layout_node(&self, id: WidgetId) -> Ref<'_, LayoutNode> {
        self.ui.layout_node(id)
    }

    pub fn request_repaint(&self) {
        self.ui.request_repaint()
    }

    pub fn time(&self) -> Duration {
        self.ui.time()
    }

    pub fn current_frame(&self) -> u64 {
        self.ui.current_frame()
    }

    pub fn debug(&mut self, text: impl ToString) {
        self.paint.debug(text)
    }

    pub fn mouse_over(&self) -> bool {
        self.ui.mouse_over()
    }

    pub fn mouse_over_widget(&self, id: WidgetId) -> bool {
        self.ui.mouse_over_widget(id)
    }

    pub fn animate_bool(&mut self, source: impl Hash, value: bool, time: f32) -> f32 {
        self.ui.animate_bool(source, value, time)
    }

    pub fn animate_value(&mut self, source: impl Hash, value: f32, time: f32) -> f32 {
        self.ui.animate_value(source, value, time)
    }
}
