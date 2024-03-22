use crate::{
    color::Color,
    geom::{pos2, rect, Align, Align2, Pos2, Rect, Vec2},
};

use super::{shape::Shape, Attribute, Cell, Label};

#[derive(Copy, Clone, Debug)]
pub struct Styled<T: Label> {
    fg: Color,
    bg: Color,
    attr: Option<Attribute>,
    align: Align2,
    pub label: T,
}

impl<T: Label + Default> Default for Styled<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Label> From<T> for Styled<T> {
    fn from(value: T) -> Self {
        Styled::new(value)
    }
}

impl<T: Label> Styled<T> {
    pub const fn new(label: T) -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reuse,
            attr: None,
            align: Align2::LEFT_TOP,
            label,
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

    pub fn bold(self) -> Self {
        self.attr(Attribute::BOLD)
    }

    pub fn faint(self) -> Self {
        self.attr(Attribute::FAINT)
    }

    pub fn italic(self) -> Self {
        self.attr(Attribute::ITALIC)
    }

    pub fn underline(self) -> Self {
        self.attr(Attribute::UNDERLINE)
    }

    pub fn blink(self) -> Self {
        self.attr(Attribute::BLINK)
    }

    pub fn reverse(self) -> Self {
        self.attr(Attribute::REVERSE)
    }

    pub fn strikeout(self) -> Self {
        self.attr(Attribute::STRIKEOUT)
    }

    pub fn attr(mut self, attr: impl Into<Option<Attribute>>) -> Self {
        self.attr = attr.into();
        self
    }

    pub const fn h_align(mut self, align: Align) -> Self {
        self.align.x = align;
        self
    }

    pub const fn v_align(mut self, align: Align) -> Self {
        self.align.y = align;
        self
    }

    pub const fn align2(mut self, align: Align2) -> Self {
        self.align = align;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.size().x == 0
    }

    pub fn size(&self) -> Vec2 {
        self.label.size()
    }

    pub fn into_static(self) -> Styled<T::Static> {
        Styled {
            fg: self.fg,
            bg: self.bg,
            attr: self.attr,
            align: self.align,
            label: self.label.into_static(),
        }
    }

    pub fn render(&self, rect: Rect, mut put: impl FnMut(Pos2, Cell)) {
        let offset = rect.left_top();

        let item_size = self.label.size();
        let x = match self.align.x {
            Align::Min => 0,
            Align::Center => (rect.width() / 2).saturating_sub(item_size.x / 2),
            Align::Max => rect.width().saturating_sub(item_size.x),
        };

        let y = match self.align.y {
            Align::Min => 0,
            Align::Center => (rect.width() / 2).saturating_sub(item_size.y / 2),
            Align::Max => rect.width().saturating_sub(item_size.y),
        };

        let mut start = pos2(x, y) + offset;
        for ch in self.label.chars() {
            if start.x >= rect.width() || start.y >= rect.width() {
                break;
            }
            if ch == '\n' {
                start.y += 1;
                start.x = x;
                continue;
            }
            put(start, Cell::new(ch).fg(self.fg).bg(self.bg).attr(self.attr));
            start.x += 1;
        }
    }
}

impl<T: Label> Shape for Styled<T> {
    fn draw(&self, size: Vec2, put: impl FnMut(Pos2, Cell)) {
        self.render(rect(size), put)
    }
}

#[track_caller]
pub fn render(data: &str, rect: Rect, mut put: impl FnMut(Pos2, Cell)) {
    let offset = rect.left_top();

    let mut start = offset;
    for ch in data.chars() {
        if start.x > rect.right() || start.y > rect.bottom() {
            break;
        }
        if ch == '\n' {
            start.y += 1;
            start.x = offset.x;
            continue;
        }

        put(start, Cell::new(ch));
        start.x += 1;
    }
}
