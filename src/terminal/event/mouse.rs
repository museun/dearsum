use crate::geom::{Pos2, Vec2};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    Move,
    Click { button: MouseButton },
    Release { button: MouseButton },
    Held { button: MouseButton },
    DragStart { button: MouseButton },
    DragHeld { delta: Vec2, button: MouseButton },
    DragRelease { button: MouseButton },
    Scroll { delta: Vec2 },
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum MouseButton {
    #[default]
    Primary,
    Secondary,
    Middle,
}

impl MouseButton {
    pub const fn is_primary(&self) -> bool {
        matches!(self, Self::Primary)
    }

    pub const fn is_secondary(&self) -> bool {
        matches!(self, Self::Secondary)
    }

    pub const fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
}

impl From<crossterm::event::MouseButton> for MouseButton {
    fn from(value: crossterm::event::MouseButton) -> Self {
        match value {
            crossterm::event::MouseButton::Left => Self::Primary,
            crossterm::event::MouseButton::Right => Self::Secondary,
            crossterm::event::MouseButton::Middle => Self::Middle,
        }
    }
}

#[derive(Default, Debug)]
enum Kind {
    #[default]
    None,
    Held,
    DragStart(Pos2),
    Drag(Pos2, Pos2),
}

#[derive(Default)]
pub struct MouseState {
    pos: Pos2,
    previous: Kind,
    button: Option<MouseButton>,
}

pub enum TemporalEvent {
    Down(Pos2, MouseButton),
    Up(Pos2, MouseButton),
    Drag(Pos2, MouseButton),
}

impl MouseState {
    pub(crate) fn update(&mut self, ev: TemporalEvent) -> Option<MouseEvent> {
        use TemporalEvent as E;
        let t = match ev {
            E::Down(pos, button) => {
                self.previous = Kind::Held;
                self.pos = pos;
                self.button = Some(button);
                MouseEvent::Held { button }
            }
            E::Up(pos, button) => match std::mem::take(&mut self.previous) {
                Kind::Held if self.check(pos, button) => {
                    self.button.take();
                    MouseEvent::Click { button }
                }
                Kind::Drag(..) if Some(button) == self.button => {
                    self.button.take();
                    MouseEvent::DragRelease { button }
                }
                _ => return None,
            },

            // TODO this is all wrong
            E::Drag(pos, button) => match std::mem::take(&mut self.previous) {
                Kind::None if self.pos == pos => {
                    self.previous = Kind::Held;
                    self.button = Some(button);
                    self.pos = pos;
                    MouseEvent::Held { button }
                }
                Kind::Held if self.pos == pos => {
                    self.previous = Kind::Held;
                    self.button = Some(button);
                    self.pos = pos;
                    return None;
                }
                Kind::None | Kind::Held => {
                    self.previous = Kind::DragStart(pos);
                    self.button = Some(button);
                    self.pos = pos;
                    MouseEvent::DragStart { button }
                }
                Kind::DragStart(origin) if self.check(origin, button) => {
                    self.previous = Kind::Drag(origin, origin);
                    self.button = Some(button);
                    self.pos = origin;
                    MouseEvent::DragHeld {
                        delta: Vec2::ZERO,
                        button,
                    }
                }
                Kind::Drag(old, origin) if self.check(origin, button) => {
                    self.previous = Kind::Drag(pos, origin);
                    self.button = Some(button);
                    self.pos = origin;
                    MouseEvent::DragHeld {
                        delta: (pos - old).to_vec2(),
                        button,
                    }
                }
                _ => return None,
            },
        };

        Some(t)
    }

    fn check(&self, pos: Pos2, button: MouseButton) -> bool {
        self.pos == pos && self.button == Some(button)
    }
}
