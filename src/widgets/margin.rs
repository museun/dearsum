use crate::{
    context::LayoutCtx,
    geom::{Constraints, Margin, Size},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug, Default)]
struct MarginWidget {
    props: Margin,
}

impl Widget for MarginWidget {
    type Response = NoResponse;
    type Props<'a> = Margin;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let margin: Size = self.props.sum().into();
        let offset = self.props.left_top().to_pos2();
        let constraints = Constraints {
            min: (input.min - margin).max(Size::ZERO),
            max: (input.max - margin).max(Size::ZERO),
        };
        let mut size = Size::ZERO;
        for &child in ctx.children {
            size = ctx.compute(child, constraints) + margin;
            ctx.set_pos(child, offset)
        }

        constraints.constrain_min(size.max(margin))
    }
}

pub fn margin<R>(margin: Margin, show: impl FnOnce() -> R) -> Response<NoResponse, R> {
    MarginWidget::show_children(margin, show)
}
