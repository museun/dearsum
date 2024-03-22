use crate::{
    context::LayoutCtx,
    geom::{Constraints, Size},
    widget::UserResponse,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Default, Debug)]
struct Sized {
    min: Option<Size>,
    max: Option<Size>,
}

#[derive(Debug)]
struct SizedWidget {
    props: Sized,
}

impl Default for SizedWidget {
    fn default() -> Self {
        Self {
            props: Sized {
                min: None,
                max: Some(Size::INFINITY),
            },
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
        if let Some(min) = self.props.min {
            input.min = input.min.min(min);
        }
        if let Some(max) = self.props.max {
            input.max = input.max.max(max);
        }

        let mut size = Size::ZERO;
        for &child in ctx.children {
            let child_size = ctx.compute(child, input);
            size = size.max(child_size)
        }
        input.constrain(size)
    }
}

fn sized_widget<R>(props: Sized, show: impl FnOnce() -> R) -> UserResponse<R> {
    SizedWidget::show_children(props, show)
}

pub fn exact_size<R>(size: impl Into<Size>, show: impl FnOnce() -> R) -> UserResponse<R> {
    let size = size.into();
    let props = Sized {
        min: Some(size),
        max: Some(size),
    };
    sized_widget(props, show)
}

pub fn min_size<R>(size: impl Into<Size>, show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Sized {
        min: Some(size.into()),
        max: None,
    };
    sized_widget(props, show)
}

pub fn max_size<R>(size: impl Into<Size>, show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Sized {
        min: None,
        max: Some(size.into()),
    };
    sized_widget(props, show)
}

pub fn max_height<R>(max_height: i32, show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Sized {
        min: None,
        max: Some(Size::new(f32::INFINITY, max_height as _)),
    };
    sized_widget(props, show)
}

pub fn max_width<R>(max_width: i32, show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Sized {
        min: None,
        max: Some(Size::new(max_width as _, f32::INFINITY)),
    };
    sized_widget(props, show)
}

pub fn expand_width<R>(show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Sized {
        min: Some(Size::new(f32::INFINITY, 0.0)),
        max: None,
    };
    sized_widget(props, show)
}

pub fn expand_height<R>(show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Sized {
        min: Some(Size::new(0.0, f32::INFINITY)),
        max: None,
    };
    sized_widget(props, show)
}
