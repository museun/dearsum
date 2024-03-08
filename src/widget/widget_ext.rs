use crate::ui;

use super::{Response, Widget};

pub trait WidgetExt: Widget + Sized {
    fn show(props: Self::Props<'_>) -> Response<Self::Response> {
        ui().widget::<Self>(props)
    }

    fn show_children<R>(
        props: Self::Props<'_>,
        show: impl FnOnce() -> R,
    ) -> Response<Self::Response> {
        let ui = ui();
        let resp = ui.begin_widget::<Self>(props);
        let _inner = show();
        ui.end_widget(resp.id());
        resp
    }
}

impl<T: Widget> WidgetExt for T {}
