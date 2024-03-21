use crate::{
    context::EventCtx,
    input::{Event, Handled, Interest, Key, Keybind, Modifiers},
    widget::Response,
    Widget, WidgetExt as _,
};

pub struct KeyAreaResponse {
    pub key: Option<Key>,
    pub modifiers: Option<Modifiers>,
}

#[derive(Debug, Default)]
pub struct KeyAreaWidget {
    last_key: Option<Key>,
    last_modifiers: Option<Modifiers>,
}

impl Widget for KeyAreaWidget {
    type Response = KeyAreaResponse;
    type Props<'a> = ();

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {
        Self::Response {
            key: std::mem::take(&mut self.last_key),
            modifiers: std::mem::take(&mut self.last_modifiers),
        }
    }

    fn interest(&self) -> Interest {
        Interest::KEY_INPUT
    }

    fn event(&mut self, _ctx: EventCtx, event: Event) -> Handled {
        if let Event::KeyInput(ev) = event {
            self.last_modifiers = Some(ev.modifiers);
            self.last_key = Some(ev.key);
        }

        Handled::Bubble
    }
}

pub fn key_area<R>(show: impl FnOnce() -> R) -> Response<KeyAreaResponse, R> {
    KeyAreaWidget::show_children((), show)
}

pub fn hot_key<R>(keybind: impl Into<Keybind>, show: impl FnOnce() -> R) -> Response<bool, R> {
    let resp = key_area(show);
    let keybind = keybind.into();
    let pressed = match (resp.key, resp.modifiers) {
        (Some(key), None) => keybind == Keybind::new(key, Modifiers::NONE),
        (Some(key), Some(modifiers)) => keybind == Keybind::new(key, modifiers),
        _ => false,
    };
    Response::new(resp.id(), pressed, resp.into_output())
}
