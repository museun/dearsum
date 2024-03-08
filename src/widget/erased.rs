use crate::{
    context::{EventCtx, LayoutCtx, PaintCtx},
    geom::{Constraints, FlexFit, Flow, Size},
    input::{Event, Handled, Interest},
};

use super::Widget;

pub trait ErasedWidget: std::any::Any + std::fmt::Debug {
    fn flex(&self) -> (u16, FlexFit);
    fn flow(&self) -> Flow;

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Size;
    fn paint(&self, ctx: PaintCtx);

    fn interest(&self) -> Interest;
    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled;

    fn default_layout(&self, ctx: LayoutCtx, input: Constraints) -> Size;
    fn default_paint(&self, ctx: PaintCtx);

    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Widget> ErasedWidget for T {
    fn flex(&self) -> (u16, FlexFit) {
        <Self as Widget>::flex(self)
    }

    fn flow(&self) -> Flow {
        <Self as Widget>::flow(self)
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Size {
        <Self as Widget>::layout(self, ctx, input)
    }

    fn paint(&self, ctx: PaintCtx) {
        <Self as Widget>::paint(self, ctx)
    }

    fn interest(&self) -> Interest {
        <Self as Widget>::interest(self)
    }

    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled {
        <Self as Widget>::event(self, ctx, event)
    }

    fn default_layout(&self, ctx: LayoutCtx, input: Constraints) -> Size {
        <Self as Widget>::default_layout(self, ctx, input)
    }

    fn default_paint(&self, ctx: PaintCtx) {
        <Self as Widget>::default_paint(self, ctx)
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as _
    }
}
