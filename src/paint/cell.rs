use crate::color::Color;

use super::Attribute;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub(crate) char: char,
    pub(crate) fg: Color,
    pub(crate) bg: Color,
    pub(crate) attr: CellAttr,
}

impl Default for Cell {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self::new(value)
    }
}

impl Cell {
    pub const EMPTY: Self = Self {
        char: ' ',
        fg: Color::Reset,
        bg: Color::Reset,
        attr: CellAttr::Reset,
    };

    pub const fn new(char: char) -> Self {
        Self {
            char,
            fg: Color::Reset,
            bg: Color::Reuse,
            attr: CellAttr::Reset,
        }
    }

    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    pub fn bg(mut self, bg: impl Into<Color>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn attr(mut self, attr: impl Into<Option<Attribute>>) -> Self {
        self.attr = attr.into().map(CellAttr::Attr).unwrap_or(CellAttr::Reset);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellAttr {
    Reset,
    Attr(Attribute),
}

impl From<Attribute> for CellAttr {
    fn from(value: Attribute) -> Self {
        Self::Attr(value)
    }
}
