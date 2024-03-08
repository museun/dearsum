use crate::terminal::geom::{Align, Margin};

use crate::prelude::*;

use super::margin;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WindowResponse {
    pub closed: bool,
}

#[derive(Debug)]
pub struct Window<T: Label> {
    title: Option<Styled<T>>,
}

impl<T: Label> Window<T> {
    pub fn new(label: impl Into<Styled<T>>) -> Window<T::Static> {
        let styled = label.into().into_static().h_align(Align::CENTER);
        Window {
            title: Some(styled),
        }
    }
}

impl<T: Label + 'static> Window<T> {
    pub fn show<R>(self, ui: &Ui, show: impl FnOnce(&Ui) -> R) -> Response<WindowResponse> {
        WindowWidget::show_children(ui, self, show)
    }
}

impl<T: Label> Default for Window<T> {
    fn default() -> Self {
        Self { title: None }
    }
}

#[derive(Debug)]
struct WindowWidget<T: Label + 'static> {
    open: bool,
    props: Window<T>,
}

impl<T: Label + 'static> Default for WindowWidget<T> {
    fn default() -> Self {
        Self {
            open: true,
            props: Window::default(),
        }
    }
}

impl<T: Label + 'static> Widget for WindowWidget<T> {
    type Response = WindowResponse;
    type Props<'a> = Window<T>;

    fn update(&mut self, props: Self::Props<'_>, ui: &Ui) -> Self::Response {
        self.props = props;
        Self::Response { closed: !self.open }
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Size {
        let mut size = Size::ZERO;
        let constraints = Constraints::tight(input.min);
        for &child in ctx.nodes.children() {
            size = size.max(ctx.layout.compute(child, constraints));
        }
        input.constrain_min(size)
    }

    fn paint(&self, mut ctx: PaintCtx) {
        ctx.draw(shape::Filled::bg(0x333333));
        if let Some(title) = &self.props.title {
            let mut canvas = ctx.crop(rect(title.size()));
            canvas.draw(shape::Filled::bg(0x555555));
            canvas.draw(title)
        }

        self.default_paint(ctx)
    }
}

pub fn window<T: Label, R>(
    ui: &Ui,
    title: impl Into<Styled<T>>,
    show: impl FnOnce(&Ui) -> R,
) -> Response<WindowResponse> {
    Window::new(title).show(ui, |ui| margin(ui, Margin::new(0, 1, 0, 0), show))
}
