use crate::{
    context::LayoutCtx,
    geom::{Constraints, Size},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Default, Debug)]
struct FloatWidget;

impl Widget for FloatWidget {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        ctx.new_layer();
        self.default_layout(ctx, Constraints::tight(input.size()))
    }
}

pub fn float<R>(show: impl FnOnce() -> R) -> Response {
    FloatWidget::show_children((), show)
}

#[derive(Default, Debug)]
struct ClipWidget;

impl Widget for ClipWidget {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        ctx.enable_clipping();
        self.default_layout(ctx, Constraints::tight(input.size()))
    }
}

pub fn clip<R>(show: impl FnOnce() -> R) -> Response {
    ClipWidget::show_children((), show)
}
