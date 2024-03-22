use crate::{
    context::{LayoutCtx, PaintCtx},
    geom::{Constraints, Pos2, Size},
    paint::{Cell, Label, MappedStyle, Styled},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

struct LabelWidget<T, F>
where
    T: Label + 'static,
    // TODO this garbage needs to go away
    F: Fn(Pos2, Cell) -> Cell + Copy + 'static,
{
    props: Option<MappedStyle<T, F>>,
}

impl<T, F> std::fmt::Debug for LabelWidget<T, F>
where
    T: Label + 'static + std::fmt::Debug,
    F: Fn(Pos2, Cell) -> Cell + Copy + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LabelWidget")
            .field("props", &self.props)
            .finish()
    }
}

impl<T, F> Default for LabelWidget<T, F>
where
    T: Label + 'static,
    F: Fn(Pos2, Cell) -> Cell + Copy + 'static,
{
    fn default() -> Self {
        Self { props: None }
    }
}

impl<T, F> Widget for LabelWidget<T, F>
where
    T: Label + 'static,
    F: Fn(Pos2, Cell) -> Cell + Copy + 'static,
{
    type Response = NoResponse;
    type Props<'a> = MappedStyle<T, F>;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = Some(props);
    }

    fn layout(&self, _: LayoutCtx, input: Constraints) -> Size {
        let Some(props) = &self.props else {
            return Size::ZERO;
        };
        input.constrain(props.label.size())
    }

    fn paint(&self, ctx: PaintCtx) {
        if let Some(props) = &self.props {
            ctx.canvas.draw(props);
        }
    }
}

// TODO partial props w/ partial equality
pub fn label<T: Label>(label: impl Into<Styled<T>>) -> Response {
    LabelWidget::show(MappedStyle::new(label).into_static())
}

pub fn mapped_label<T, F>(label: impl Into<Styled<T>>, map: F) -> Response
where
    T: Label,
    F: Fn(Pos2, Cell) -> Cell + 'static + Copy,
{
    LabelWidget::show(MappedStyle::new(label).map(map).into_static())
}
