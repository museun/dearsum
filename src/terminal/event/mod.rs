use crate::geom::{pos2, rect, vec2, Pos2, Rect};

mod keyboard;
pub use keyboard::{Key, Keybind};

mod mouse;
use mouse::TemporalEvent;
pub use mouse::{MouseButton, MouseEvent, MouseState};

mod modifiers;
pub use modifiers::Modifiers;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Keyboard(Key, Modifiers),
    Mouse(MouseEvent, Pos2, Modifiers),
    Resize(Rect),
    Quit,
    FocusGained,
    FocusLost,
    Paste(String),
}

// TODO this needs to read before the timer and still advance the timer. and then read after the timer
pub fn read_next_event(state: &mut MouseState) -> Option<Event> {
    const NO_TIMEOUT: std::time::Duration = std::time::Duration::ZERO;
    match crossterm::event::poll(NO_TIMEOUT) {
        Ok(true) => {
            let Ok(ev) = crossterm::event::read() else {
                return Some(Event::Quit);
            };
            translate(ev, state)
        }
        Err(..) => Some(Event::Quit),
        _ => None,
    }
}

fn translate(ev: crossterm::event::Event, state: &mut MouseState) -> Option<Event> {
    use crossterm::event::{Event as E, KeyEventKind, MouseEventKind as M};

    let ev = match ev {
        E::FocusGained => Event::FocusGained,
        E::FocusLost => Event::FocusLost,
        E::Paste(data) => Event::Paste(data),
        E::Resize(cols, rows) => Event::Resize(rect(vec2(cols as _, rows as _))),

        E::Key(ev) if matches!(ev.kind, KeyEventKind::Release) => {
            let key = ev.code.try_into().ok()?;
            let modifiers = ev.modifiers.into();
            Event::Keyboard(key, modifiers)
        }

        E::Mouse(ev) => {
            let pos = pos2(ev.column as _, ev.row as _);
            let modifiers = ev.modifiers.into();

            let event = match ev.kind {
                M::Down(button) => state.update(TemporalEvent::Down(pos, button.into()))?,
                M::Up(button) => state.update(TemporalEvent::Up(pos, button.into()))?,
                M::Drag(button) => state.update(TemporalEvent::Drag(pos, button.into()))?,

                M::Moved => MouseEvent::Move,

                M::ScrollDown => MouseEvent::Scroll { delta: vec2(0, 1) },
                M::ScrollUp => MouseEvent::Scroll { delta: vec2(0, -1) },
                M::ScrollLeft => MouseEvent::Scroll { delta: vec2(-1, 0) },
                M::ScrollRight => MouseEvent::Scroll { delta: vec2(1, 0) },
            };

            Event::Mouse(event, pos, modifiers)
        }
        _ => return None,
    };

    Some(ev)
}
