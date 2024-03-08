use crate::{
    context::LayoutCtx,
    geom::{Constraints, Size},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug)]
struct ConstrainedWidget {
    props: Constraints,
}

impl Default for ConstrainedWidget {
    fn default() -> Self {
        Self {
            props: Constraints {
                min: Size::ZERO,
                max: Size::ZERO,
            },
        }
    }
}

impl Widget for ConstrainedWidget {
    type Response = NoResponse;
    type Props<'a> = Constraints;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let constraints = Constraints {
            min: input.min.max(self.props.min),
            max: input.max.max(self.props.max),
        };
        let mut size = Size::ZERO;
        for &child in ctx.children {
            size = size.max(ctx.compute(child, constraints))
        }
        input.constrain(constraints.constrain(size))
    }
}

pub fn constrained<R>(constraints: Constraints, show: impl FnOnce() -> R) -> Response {
    ConstrainedWidget::show_children(constraints, show)
}

#[derive(Default, Debug)]
pub struct Unconstrained {
    constrain_x: bool,
    constrain_y: bool,
}

impl Unconstrained {
    pub const fn new() -> Self {
        Self {
            constrain_x: false,
            constrain_y: false,
        }
    }

    pub const fn constrain_x(mut self, constrain_x: bool) -> Self {
        self.constrain_x = constrain_x;
        self
    }

    pub const fn constrain_y(mut self, constrain_y: bool) -> Self {
        self.constrain_y = constrain_y;
        self
    }

    pub fn show<R>(self, show: impl FnOnce() -> R) -> Response {
        UnconstainedWidget::show_children(self, show)
    }
}

#[derive(Default, Debug)]
struct UnconstainedWidget {
    props: Unconstrained,
}

impl Widget for UnconstainedWidget {
    type Response = NoResponse;
    type Props<'a> = Unconstrained;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let max_x = if self.props.constrain_x {
            input.max.x
        } else {
            f32::INFINITY
        };
        let max_y = if self.props.constrain_y {
            input.max.y
        } else {
            f32::INFINITY
        };

        let constraints = Constraints {
            min: Size::ZERO,
            max: Size::new(max_x, max_y),
        };

        let mut size = Size::ZERO;
        for &child in ctx.children {
            size = size.max(ctx.compute(child, constraints))
        }
        input.constrain_min(size)
    }
}

pub fn unconstrained<R>(show: impl FnOnce() -> R) -> Response {
    Unconstrained::new().show(show)
}
