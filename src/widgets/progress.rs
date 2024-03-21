use std::ops::RangeInclusive;

use crate::{
    color::Rgba,
    context::{LayoutCtx, PaintCtx},
    geom::{math::remap, size, vec2, Constraints, Rect, Size},
    paint::{shape::Filled, Cell},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Default, Debug)]
pub struct Progress {
    pos: f32,
    min: f32,
    max: f32,
    min_size: Size,

    bg: Rgba,
    filled: Rgba,
}

impl Progress {
    pub const fn new(pos: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            pos,
            min: *range.start(),
            max: *range.end(),

            min_size: size(20.0, 1.0),

            bg: Rgba::from_u32(0x222222),
            filled: Rgba::from_u32(0xAAAAAA),
        }
    }

    pub fn bg(mut self, bg: impl Into<Rgba>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn filled(mut self, filled: impl Into<Rgba>) -> Self {
        self.filled = filled.into();
        self
    }

    pub fn show(self) -> Response {
        ProgressWidget::show(self)
    }
}

#[derive(Default, Debug)]
struct ProgressWidget {
    props: Progress,
}

impl Widget for ProgressWidget {
    type Response = NoResponse;
    type Props<'a> = Progress;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _: LayoutCtx, input: Constraints) -> Size {
        input.constrain_min(self.props.min_size)
    }

    fn paint(&self, mut ctx: PaintCtx) {
        let rect = ctx.rect;

        ctx.draw(Filled::bg(self.props.bg));

        let (min, max) = (rect.left(), rect.right() + 1);
        let x = remap(self.props.pos, (self.props.min, self.props.max), (min, max)) - min;

        ctx.draw_cropped(
            Rect::from_min_size(ctx.rect.min, vec2(x, 1)),
            Filled::new(Cell::new(' ').bg(self.props.filled)),
        );
    }
}

pub fn progress(pos: f32, range: RangeInclusive<f32>) -> Response {
    Progress::new(pos, range).show()
}
