use super::{vec2, Rect, Vec2};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Margin {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

impl Margin {
    pub const ZERO: Self = Self::same(0);
    pub const ONE: Self = Self::same(1);

    pub const fn new(left: u16, top: u16, right: u16, bottom: u16) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn symmetric(x: u16, y: u16) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }

    pub const fn same(margin: u16) -> Self {
        Self::symmetric(margin, margin)
    }

    pub const fn sum(&self) -> Vec2 {
        vec2(
            self.left as i32 + self.right as i32,
            self.top as i32 + self.bottom as i32,
        )
    }

    pub const fn left_top(&self) -> Vec2 {
        vec2(self.left as i32, self.top as i32)
    }

    pub const fn right_bottom(&self) -> Vec2 {
        vec2(self.right as i32, self.bottom as i32)
    }

    pub const fn is_same(&self) -> bool {
        self.left == self.right && self.left == self.top && self.left == self.bottom
    }

    pub fn expand_rect(&self, rect: Rect) -> Rect {
        Rect::from_min_max(
            rect.min - self.left_top(), //
            rect.max + self.right_bottom(),
        )
    }

    pub fn shrink_rect(&self, rect: Rect) -> Rect {
        Rect::from_min_max(
            rect.min + self.left_top(), //
            rect.max - self.right_bottom(),
        )
    }
}

impl From<(u16, u16)> for Margin {
    fn from((x, y): (u16, u16)) -> Self {
        Self::symmetric(x, y)
    }
}

impl From<u16> for Margin {
    fn from(value: u16) -> Self {
        Self::same(value)
    }
}

impl From<Vec2> for Margin {
    fn from(value: Vec2) -> Self {
        Self::symmetric(value.x as u16, value.y as u16)
    }
}
