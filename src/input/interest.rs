#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Interest(u8);

impl Interest {
    pub const NONE: Self = Self(0);

    pub const MOUSE_ENTER: Self = Self(1 << 0);
    pub const MOUSE_LEAVE: Self = Self(1 << 1);
    pub const MOUSE_MOVE: Self = Self(1 << 2);

    pub const KEY_INPUT: Self = Self(1 << 3);

    pub const FOCUS_GAINED: Self = Self(1 << 4);
    pub const FOCUS_LOST: Self = Self(1 << 5);

    pub const MOUSE: Self = Self(Self::MOUSE_ENTER.0 | Self::MOUSE_LEAVE.0 | Self::MOUSE_MOVE.0);
    pub const FOCUS: Self = Self(Self::FOCUS_GAINED.0 | Self::FOCUS_LOST.0);

    pub fn is_mouse_any(&self) -> bool {
        self.is_mouse_enter() || self.is_mouse_leave() || self.is_mouse_move()
    }

    pub fn is_focus(&self) -> bool {
        self.is_focus_gained() || self.is_focus_lost()
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub fn is_mouse_enter(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    pub fn is_mouse_leave(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub fn is_mouse_move(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub fn is_key_input(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub fn is_focus_gained(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    pub fn is_focus_lost(&self) -> bool {
        self.0 & (1 << 5) != 0
    }
}

impl std::fmt::Binary for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl std::fmt::Debug for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static FIELDS: [&str; 6] = [
            "MOUSE_ENTER",
            "MOUSE_LEAVE",
            "MOUSE_MOVE",
            "KEY_INPUT",
            "FOCUS_GAINED",
            "FOCUS_LOST",
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

        if !seen {
            f.write_str("NONE")?
        }

        Ok(())
    }
}

impl std::ops::BitOr for Interest {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Interest {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::Not for Interest {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
