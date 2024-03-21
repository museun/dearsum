use crate::{geom::Flow, widget::Response, NoResponse, Widget, WidgetExt as _};

#[derive(Debug)]
struct FlowWidget {
    props: Flow,
}

impl Default for FlowWidget {
    fn default() -> Self {
        Self {
            props: Flow::Inline,
        }
    }
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

pub fn flow<R>(flow: Flow, show: impl FnOnce() -> R) -> Response<NoResponse, R> {
    FlowWidget::show_children(flow, show)
}
