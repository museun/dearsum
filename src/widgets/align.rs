use crate::{
    context::LayoutCtx,
    geom::{Align2, Constraints, Size},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug, Default)]
struct AlignWidget {
    align: Align2,
}

impl Widget for AlignWidget {
    type Response = NoResponse;
    type Props<'a> = Align2;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.align = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let constraints = Constraints::loose(input.max);
        let mut size = input.size();

        for &child in ctx.children {
            let next = ctx.compute(child, constraints);
            size = size.max(next);
            let pos = (size * self.align - next * self.align).to_pos2();
            ctx.set_pos(child, pos);
        }

        size
    }
}

pub fn align<R>(align: Align2, show: impl FnOnce() -> R) -> Response {
    AlignWidget::show_children(align, show)
}

pub fn center<R>(show: impl FnOnce() -> R) -> Response {
    self::align(Align2::CENTER_CENTER, show)
}
