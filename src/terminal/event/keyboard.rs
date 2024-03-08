use super::Modifiers;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    Function(u8),
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Enter,
    Delete,
    Backspace,
    Escape,
    Tab,
    BackTab,
}

impl TryFrom<crossterm::event::KeyCode> for Key {
    type Error = crossterm::event::KeyCode;
    fn try_from(value: crossterm::event::KeyCode) -> Result<Self, Self::Error> {
        match value {
            crossterm::event::KeyCode::Char(ch) => Ok(Self::Char(ch)),
            crossterm::event::KeyCode::F(f) => Ok(Self::Function(f)),
            crossterm::event::KeyCode::Left => Ok(Self::Left),
            crossterm::event::KeyCode::Right => Ok(Self::Right),
            crossterm::event::KeyCode::Up => Ok(Self::Up),
            crossterm::event::KeyCode::Down => Ok(Self::Down),
            crossterm::event::KeyCode::PageUp => Ok(Self::PageUp),
            crossterm::event::KeyCode::Home => Ok(Self::Home),
            crossterm::event::KeyCode::End => Ok(Self::End),
            crossterm::event::KeyCode::Enter => Ok(Self::Enter),
            crossterm::event::KeyCode::Insert => Ok(Self::Insert),
            crossterm::event::KeyCode::Delete => Ok(Self::Delete),
            crossterm::event::KeyCode::Backspace => Ok(Self::Backspace),
            crossterm::event::KeyCode::Esc => Ok(Self::Escape),
            crossterm::event::KeyCode::Tab => Ok(Self::Tab),
            crossterm::event::KeyCode::BackTab => Ok(Self::BackTab),
            _ => Err(value),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Keybind {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl Keybind {
    pub const fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    pub const fn key(key: Key) -> Self {
        Self::new(key, Modifiers::NONE)
    }

    pub const fn char(char: char) -> Self {
        Self::key(Key::Char(char))
    }

    pub const fn ctrl(mut self) -> Self {
        self.modifiers = Modifiers(self.modifiers.0 | Modifiers::CTRL.0);
        self
    }

    pub const fn shift(mut self) -> Self {
        self.modifiers = Modifiers(self.modifiers.0 | Modifiers::SHIFT.0);
        self
    }

    pub const fn alt(mut self) -> Self {
        self.modifiers = Modifiers(self.modifiers.0 | Modifiers::ALT.0);
        self
    }
}

impl From<char> for Keybind {
    fn from(value: char) -> Self {
        Self::new(Key::Char(value), Modifiers::NONE)
    }
}
