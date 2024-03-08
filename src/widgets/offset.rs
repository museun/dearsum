use crate::{
    context::LayoutCtx,
    geom::{Constraints, Pos2, Size},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug, Default)]
struct OffsetWidget {
    props: Pos2,
}

impl Widget for OffsetWidget {
    type Response = NoResponse;
    type Props<'a> = Pos2;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let constraints = Constraints::loose(input.max);
        let mut size = input.size();
        for &child in ctx.children {
            size = size.max(ctx.compute(child, constraints));
            ctx.set_pos(child, self.props)
        }
        size
    }
}

pub fn offset<R>(pos: Pos2, show: impl FnOnce() -> R) -> Response {
    OffsetWidget::show_children(pos, show)
}
