use crate::{
    color::Color,
    geom::{pos2, rect, Margin, Pos2, Vec2},
    paint::Cell,
};

use super::Shape;

#[derive(Copy, Clone, Debug)]
pub struct Border {
    pub left_top: char,
    pub right_top: char,
    pub right_bottom: char,
    pub left_bottom: char,
    pub top: char,
    pub right: char,
    pub bottom: char,
    pub left: char,

    pub fg: Color,
    pub bg: Color,
}

impl Shape for Border {
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        let [left_top, right_top, right_bottom, left_bottom] = rect(size).corners();

        let top = Cell::new(self.top).fg(self.fg).bg(self.bg);
        let bottom = Cell::new(self.bottom).fg(self.fg).bg(self.bg);
        let right = Cell::new(self.right).fg(self.fg).bg(self.bg);
        let left = Cell::new(self.left).fg(self.fg).bg(self.bg);

        for x in left_top.x..right_top.x {
            put(pos2(x, left_top.y), top);
        }

        for x in left_bottom.x..right_bottom.x {
            put(pos2(x, left_bottom.y), bottom);
        }

        for y in right_top.y..right_bottom.y {
            put(pos2(right_top.x, y), right);
        }

        for y in left_top.y..left_bottom.y {
            put(pos2(left_top.x, y), left);
        }

        for (pos, cell) in [
            (left_top, self.left_top),
            (right_top, self.right_top),
            (right_bottom, self.right_bottom),
            (left_bottom, self.left_bottom),
        ] {
            put(pos, Cell::new(cell).fg(self.fg).bg(self.bg));
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::THIN
    }
}

impl Border {
    pub fn fg(mut self, fg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self
    }

    pub fn bg(mut self, bg: impl Into<Color>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn as_margin(&self) -> Margin {
        Margin {
            left: (self.left != ' ' || self.left_top != ' ' || self.left_bottom != ' ') as _,
            top: (self.top != ' ' || self.left_top != ' ' || self.right_top != ' ') as _,
            right: (self.right != ' ' || self.right_bottom != ' ' || self.right_bottom != ' ') as _,
            bottom: (self.bottom != ' ' || self.left_bottom != ' ' || self.right_bottom != ' ')
                as _,
        }
    }
}

impl Border {
    pub const EMPTY: Self = Self {
        left_top: ' ',
        top: ' ',
        right_top: ' ',
        right: ' ',
        right_bottom: ' ',
        bottom: ' ',
        left_bottom: ' ',
        left: ' ',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THIN: Self = Self {
        left_top: '┌',
        top: '─',
        right_top: '┐',
        right: '│',
        right_bottom: '┘',
        bottom: '─',
        left_bottom: '└',
        left: '│',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THIN_TALL: Self = Self {
        left_top: '▔',
        top: '▔',
        right_top: '▔',
        right: '▕',
        right_bottom: '▁',
        bottom: '▁',
        left_bottom: '▁',
        left: '▏',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THIN_WIDE: Self = Self {
        left_top: '▁',
        top: '▁',
        right_top: '▁',
        right: '▕',
        right_bottom: '▔',
        bottom: '▔',
        left_bottom: '▔',
        left: '▏',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const ROUNDED: Self = Self {
        left_top: '╭',
        top: '─',
        right_top: '╮',
        right: '│',
        right_bottom: '╯',
        bottom: '─',
        left_bottom: '╰',
        left: '│',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const DOUBLE: Self = Self {
        left_top: '╔',
        top: '═',
        right_top: '╗',
        right: '║',
        right_bottom: '╝',
        bottom: '═',
        left_bottom: '╚',
        left: '║',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THICK: Self = Self {
        left_top: '┏',
        top: '━',
        right_top: '┓',
        right: '┃',
        right_bottom: '┛',
        bottom: '━',
        left_bottom: '┗',
        left: '┃',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THICK_TALL: Self = Self {
        left_top: '▛',
        top: '▀',
        right_top: '▜',
        right: '▐',
        right_bottom: '▟',
        bottom: '▄',
        left_bottom: '▙',
        left: '▌',

        fg: Color::Reset,
        bg: Color::Reuse,
    };

    pub const THICK_WIDE: Self = Self {
        left_top: '▗',
        top: '▄',
        right_top: '▖',
        right: '▌',
        right_bottom: '▘',
        bottom: '▀',
        left_bottom: '▝',
        left: '▐',

        fg: Color::Reset,
        bg: Color::Reuse,
    };
}
