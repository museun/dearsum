use crate::{
    geom::FlexFit,
    widget::{Response, UserResponse},
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Debug)]
struct Flex {
    flex: FlexFit,
    factor: u16,
}

#[derive(Debug, Default)]
struct FlexWidget {
    flex: FlexFit,
    factor: u16,
}

impl Widget for FlexWidget {
    type Response = NoResponse;
    type Props<'a> = Flex;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.flex = props.flex;
        self.factor = props.factor;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.factor, self.flex)
    }
}

pub fn flex<R>(factor: u16, show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Flex {
        factor,
        flex: FlexFit::Loose,
    };
    FlexWidget::show_children(props, show)
}

pub fn expanded<R>(show: impl FnOnce() -> R) -> UserResponse<R> {
    let props = Flex {
        factor: 1,
        flex: FlexFit::Tight,
    };
    FlexWidget::show_children(props, show)
}

pub fn spacer() -> Response {
    let props = Flex {
        factor: 1,
        flex: FlexFit::Tight,
    };
    FlexWidget::show(props)
}
