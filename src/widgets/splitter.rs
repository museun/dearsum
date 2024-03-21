use crate::{
    color::Rgba,
    context::{EventCtx, LayoutCtx, PaintCtx},
    geom::{math::inverse_lerp, size, Axis, Constraints, Pos2, Rect, Size},
    input::{Event, Handled, Interest},
    paint::{shape::Filled, Cell},
    ui,
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

use super::{column, row, state};

// TODO this type is kind of unneeded
#[derive(Default, Debug)]
struct PaneWidget {
    props: Rect,
}

impl Widget for PaneWidget {
    type Response = NoResponse;
    type Props<'a> = Rect;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let constraints = Constraints::tight(self.props.size().into());
        for &child in ctx.children {
            ctx.compute(child, constraints);
        }
        input.constrain_min(self.props.size().into())
    }
}

fn pane<R>(rect: Rect, show: impl FnOnce() -> R) -> Response<NoResponse, R> {
    PaneWidget::show_children(rect, show)
}

#[derive(Debug, Default)]
struct SplitterWidget {
    axis: Axis,
    rect: Rect,
    pos: Option<Pos2>,
}

impl Widget for SplitterWidget {
    type Response = Option<Pos2>;
    type Props<'a> = (Axis, Rect);

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        (self.axis, self.rect) = props;
        self.pos.take()
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE
    }

    fn event(&mut self, _ctx: EventCtx, event: Event) -> Handled {
        let Event::MouseDrag(ev) = event else {
            return Handled::Bubble;
        };

        self.pos = Some(ev.pos);
        Handled::Sink
    }

    fn layout(&self, _ctx: LayoutCtx, _input: Constraints) -> Size {
        match self.axis {
            Axis::Horizontal => size(1.0, self.rect.size().y as f32),
            Axis::Vertical => size(self.rect.size().x as f32, 1.0),
        }
    }

    fn paint(&self, mut ctx: PaintCtx) {
        let ch = match self.axis {
            Axis::Horizontal => '│',
            Axis::Vertical => '─',
        };

        ctx.draw(Filled::new(
            Cell::new(ch)
                .fg(if ctx.mouse_over() { 0xFFFF00 } else { u32::MAX })
                .bg(Rgba::from_u32(u32::MIN).with_alpha(0x55)),
        ));
    }
}

pub fn split<L, R>(
    axis: Axis,
    primary: impl FnOnce() -> L,
    secondary: impl FnOnce() -> R,
) -> Response {
    let ui = ui();
    let split = state(|| 0.5);
    let rect = ui.available_rect();

    let (main, cross) = match axis {
        Axis::Horizontal => rect.split_horizontal_ratio(1, split.get()),
        Axis::Vertical => rect.split_vertical_ratio(1, split.get()),
    };

    let show = || {
        // this is just a constrained widget
        pane(main, primary);

        if let Some(pos) = *ui.widget::<SplitterWidget>((axis, rect)) {
            let (x, y, t) = match axis {
                Axis::Horizontal => (rect.left(), rect.right(), pos.x),
                Axis::Vertical => (rect.top(), rect.bottom(), pos.y),
            };
            split.set(inverse_lerp(x as f32, y as f32, t as f32).unwrap_or(0.5));
        }

        // this is just a constrained widget
        pane(cross, secondary);
    };

    match axis {
        Axis::Horizontal => row(show),
        Axis::Vertical => column(show),
    }
}

pub fn split_vertical<L, R>(
    primary: impl FnOnce() -> L,
    secondary: impl FnOnce() -> R,
) -> Response {
    split(Axis::Vertical, primary, secondary)
}

pub fn split_horizontal<L, R>(
    primary: impl FnOnce() -> L,
    secondary: impl FnOnce() -> R,
) -> Response {
    split(Axis::Horizontal, primary, secondary)
}
