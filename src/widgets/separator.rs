use crate::{
    context::{LayoutCtx, PaintCtx},
    geom::{size, Constraints, Size},
    paint::{shape::Filled, Cell},
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug, Default)]
struct SeparatorWidget {
    props: Separator,
}

impl Widget for SeparatorWidget {
    type Response = NoResponse;
    type Props<'a> = Separator;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _ctx: LayoutCtx, input: Constraints) -> Size {
        input.constrain_min(size(input.size().x, 1.0))
    }

    fn paint(&self, mut ctx: PaintCtx) {
        ctx.draw(Filled::new(self.props.cell))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Separator {
    cell: Cell,
}

impl Default for Separator {
    fn default() -> Self {
        Self { cell: '─'.into() }
    }
}

impl Separator {
    pub fn show(cell: impl Into<Cell>) {
        SeparatorWidget::show(Self { cell: cell.into() });
    }
}

pub fn separator() {
    Separator::show('─')
}
