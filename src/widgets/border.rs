use crate::{
    context::PaintCtx,
    geom::{vec2, Margin},
    paint::{shape, Label, Styled},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

use super::margin;

#[derive(Debug)]
struct Border<T: Label + 'static = ()> {
    style: shape::Border,
    title: Option<Styled<T>>,
}

impl<T: Label + 'static> Default for Border<T> {
    fn default() -> Self {
        Self {
            style: Default::default(),
            title: None,
        }
    }
}

impl<T: Label> Border<T> {
    pub fn show<R>(self, children: impl FnOnce() -> R) -> Response<NoResponse, R> {
        <BorderWidget<T>>::show_children(self, children)
    }
}

impl Border<()> {
    pub fn new<T: Label>(style: shape::Border, title: impl Into<Styled<T>>) -> Border<T::Static> {
        Border {
            style,
            title: Some(title.into().into_static()),
        }
    }
}

impl Border {
    pub fn plain(style: shape::Border) -> Border<()> {
        Border { style, title: None }
    }
}

#[derive(Debug)]
struct BorderWidget<T: Label + 'static = ()> {
    props: Border<T>,
}

impl<T: Label + 'static> Default for BorderWidget<T> {
    fn default() -> Self {
        Self {
            props: Border::default(),
        }
    }
}

impl<T: Label + 'static> Widget for BorderWidget<T> {
    type Response = NoResponse;
    type Props<'a> = Border<T>;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn paint(&self, ctx: PaintCtx) {
        ctx.canvas.draw(self.props.style);
        if let Some(title) = &self.props.title {
            ctx.canvas.crop(ctx.rect.shrink2(vec2(1, 0))).draw(title)
        }
        self.default_paint(ctx)
    }
}

pub fn border<R>(style: shape::Border, children: impl FnOnce() -> R) -> Response<NoResponse, R> {
    Border::plain(style).show(|| {
        margin(Margin::same(1), children).into_output() //
    })
}

pub fn frame<R, L: Label>(
    style: shape::Border,
    title: impl Into<Styled<L>>,
    children: impl FnOnce() -> R,
) -> Response<NoResponse, R> {
    let mut border_margin = style.as_margin();
    let title = title.into();
    if !title.is_empty() {
        border_margin.top = border_margin.top.max(1);
    }
    Border::new(style, title).show(|| {
        margin(border_margin, children).into_output() //
    })
}
