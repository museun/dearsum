use crate::{
    color::Rgba,
    context::{LayoutCtx, PaintCtx},
    geom::{Constraints, Size},
    paint::{shape::Filled, Cell},
    widget::{Response, UserResponse},
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Default, Debug)]
struct FilledWidget {
    props: Filled,
    min_size: Size,
}

impl Widget for FilledWidget {
    type Response = NoResponse;
    type Props<'a> = (Filled, Size);

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        (self.props, self.min_size) = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let mut size = self.min_size.min(input.max);
        for &child in ctx.children {
            size = size.max(ctx.compute(child, input))
        }
        input.constrain_min(size)
    }

    fn paint(&self, mut ctx: PaintCtx) {
        ctx.draw(self.props);
        self.default_paint(ctx)
    }
}

pub fn render_cell(cell: impl Into<Cell>) -> Response {
    FilledWidget::show((Filled::new(cell.into()), Size::ZERO))
}

pub fn filled<R>(bg: impl Into<Rgba>, show: impl FnOnce() -> R) -> UserResponse<R> {
    FilledWidget::show_children((Filled::bg(bg), Size::ZERO), show)
}

pub fn filled_rect(bg: impl Into<Rgba>, min_size: Size) -> Response {
    FilledWidget::show((Filled::bg(bg), min_size))
}
