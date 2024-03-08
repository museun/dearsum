#[derive(Copy, Clone, PartialEq, Default)]
pub struct MouseEventFilter(u8);

impl std::fmt::Debug for MouseEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static FIELDS: [&str; 7] = [
            "ENTER",  //
            "LEAVE",  //
            "MOVE",   //
            "DRAG",   //
            "CLICK",  //
            "HELD",   //
            "SCROLL", //
        ];

        let mut seen = false;
        for (flag, repr) in (0..).zip(FIELDS) {
            if (self.0 >> flag) & 1 == 1 {
                if seen {
                    f.write_str(" | ")?;
                }
                f.write_str(repr)?;
                seen |= true
            }
        }

        Ok(())
    }
}

impl std::fmt::Binary for MouseEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl MouseEventFilter {
    pub const EMPTY: Self = Self(0);

    pub const ALL: Self = Self(
        Self::ENTER.0
            | Self::LEAVE.0
            | Self::MOVE.0
            | Self::DRAG.0
            | Self::CLICK.0
            | Self::HELD.0
            | Self::SCROLL.0,
    );

    pub const ENTER: Self = Self(1 << 0);
    pub const LEAVE: Self = Self(1 << 1);
    pub const MOVE: Self = Self(1 << 2);
    pub const DRAG: Self = Self(1 << 3);
    pub const CLICK: Self = Self(1 << 4);
    pub const HELD: Self = Self(1 << 5);
    pub const SCROLL: Self = Self(1 << 5);
}

impl MouseEventFilter {
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    pub const fn enter(self) -> Self {
        Self(self.0 | Self::ENTER.0)
    }

    pub const fn leave(self) -> Self {
        Self(self.0 | Self::LEAVE.0)
    }

    pub const fn moved(self) -> Self {
        Self(self.0 | Self::MOVE.0)
    }

    pub const fn drag(self) -> Self {
        Self(self.0 | Self::DRAG.0)
    }

    pub const fn click(self) -> Self {
        Self(self.0 | Self::CLICK.0)
    }

    pub const fn held(self) -> Self {
        Self(self.0 | Self::HELD.0)
    }
}

impl MouseEventFilter {
    pub const fn is_enter(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    pub const fn is_leave(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub const fn is_move(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub const fn is_drag(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub const fn is_click(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    pub const fn is_held(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    pub const fn is_scroll(&self) -> bool {
        self.0 & (1 << 6) != 0
    }
}

impl std::ops::BitOr for MouseEventFilter {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for MouseEventFilter {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for MouseEventFilter {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for MouseEventFilter {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
