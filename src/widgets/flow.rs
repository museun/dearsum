use crate::{geom::Flow, widget::UserResponse, NoResponse, Widget, WidgetExt as _};

#[derive(Debug, Default)]
struct FlowWidget {
    props: Flow,
}

impl Widget for FlowWidget {
    type Response = NoResponse;
    type Props<'a> = Flow;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flow(&self) -> Flow {
        self.props
    }
}

pub fn flow<R>(flow: Flow, show: impl FnOnce() -> R) -> UserResponse<R> {
    FlowWidget::show_children(flow, show)
}
