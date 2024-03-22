use crate::geom::{Pos2, Vec2};
use crate::terminal::event::{Key, Modifiers, MouseButton};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
    MouseEnter(MouseMove),
    MouseLeave(MouseMove),
    MouseMove(MouseMove),
    MouseClick(MouseClick),
    MouseHeld(MouseHeld),
    MouseDrag(MouseDrag),
    MouseScroll(MouseScroll),
    KeyInput(KeyPressed),
    FocusGained,
    FocusLost,
}

impl Event {
    pub fn modifiers(&self) -> Option<Modifiers> {
        match self {
            Self::MouseClick(MouseClick { modifiers, .. })
            | Self::MouseHeld(MouseHeld { modifiers, .. })
            | Self::MouseDrag(MouseDrag { modifiers, .. })
            | Self::MouseScroll(MouseScroll { modifiers, .. })
            | Self::KeyInput(KeyPressed { modifiers, .. }) => Some(*modifiers),
            _ => return None,
        }
        .filter(|c| !c.is_none())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyPressed {
    pub key: Key,
    pub modifiers: Modifiers,
}

// TODO `inside` | `outside` for these
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseMove {
    pub pos: Pos2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseClick {
    pub pos: Pos2,
    pub button: MouseButton,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseHeld {
    pub pos: Pos2,
    pub button: MouseButton,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseDrag {
    pub released: bool,
    pub origin: Pos2,
    pub pos: Pos2,
    pub delta: Vec2,
    pub button: MouseButton,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseScroll {
    pub pos: Pos2,
    pub delta: Vec2,
    pub modifiers: Modifiers,
}
