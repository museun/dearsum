use crate::ui;

use super::{Response, Widget};

pub trait WidgetExt: Widget + Sized {
    fn show(props: Self::Props<'_>) -> Response<Self::Response> {
        ui().widget::<Self>(props)
    }

    fn show_children<R>(
        props: Self::Props<'_>,
        show: impl FnOnce() -> R,
    ) -> Response<Self::Response, R> {
        let ui = ui();
        let resp = ui.begin_widget::<Self>(props);
        let output = show();
        ui.end_widget(resp.id());
        resp.map(output)
    }
}

impl<T: Widget> WidgetExt for T {}
