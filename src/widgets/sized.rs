use crate::{
    context::LayoutCtx,
    geom::{size, Constraints, Size},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug)]
pub struct Sized {
    min: Size,
    max: Size,
}

impl Sized {
    pub const fn new(min: Size, max: Size) -> Self {
        Self { min, max }
    }

    pub const fn exact(size: Size) -> Self {
        Self {
            min: size,
            max: size,
        }
    }

    pub const fn max(max: Size) -> Self {
        Self::new(Size::ZERO, max)
    }

    pub const fn min(min: Size) -> Self {
        Self::new(min, Size::INFINITY)
    }

    pub const fn min_height(min_height: f32) -> Self {
        Self::min(size(f32::INFINITY, min_height))
    }

    pub const fn min_width(min_width: f32) -> Self {
        Self::min(size(min_width, f32::INFINITY))
    }

    pub const fn max_height(max_height: f32) -> Self {
        Self::max(size(f32::INFINITY, max_height))
    }

    pub const fn max_width(max_width: f32) -> Self {
        Self::max(size(max_width, f32::INFINITY))
    }

    pub fn show<R>(self, show: impl FnOnce() -> R) -> Response<NoResponse, R> {
        SizedWidget::show_children(self, show)
    }
}

#[derive(Debug)]
struct SizedWidget {
    props: Sized,
}

impl Default for SizedWidget {
    fn default() -> Self {
        Self {
            props: Sized::new(Size::ZERO, Size::INFINITY),
        }
    }
}

impl Widget for SizedWidget {
    type Response = NoResponse;
    type Props<'a> = Sized;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, mut input: Constraints) -> Size {
        input.min = input.min.max(self.props.min);
        input.max = input.max.min(self.props.max);

        let mut size = Size::ZERO;
        for &child in ctx.children {
            size = size.max(ctx.compute(child, input))
        }
        size
    }
}

pub fn min_size<R>(min_size: Size, show: impl FnOnce() -> R) -> Response<NoResponse, R> {
    Sized::min(min_size).show(show)
}

pub fn max_size<R>(max_size: Size, show: impl FnOnce() -> R) -> Response<NoResponse, R> {
    Sized::max(max_size).show(show)
}
