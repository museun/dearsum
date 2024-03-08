#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Modifiers(pub u8);

impl Modifiers {
    pub const NONE: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CTRL: Self = Self(1 << 1);
    pub const ALT: Self = Self(1 << 2);
}

impl Modifiers {
    pub const fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub const fn is_shift(&self) -> bool {
        self.0 & 1 == Self::SHIFT.0
    }

    pub const fn is_shift_only(&self) -> bool {
        self.0 == Self::SHIFT.0
    }

    pub const fn is_ctrl(&self) -> bool {
        (self.0 >> 1) & 1 == 1
    }

    pub const fn is_ctrl_only(&self) -> bool {
        self.0 == Self::CTRL.0
    }

    pub const fn is_alt(&self) -> bool {
        (self.0 >> 2) & 1 == 1
    }

    pub const fn is_alt_only(&self) -> bool {
        self.0 == Self::ALT.0
    }
}

impl std::ops::BitAnd for Modifiers {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitAndAssign for Modifiers {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Modifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl std::ops::Not for Modifiers {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut seen = false;
        for (i, repr) in (0..).zip(["Shift", "Ctrl", "Alt"]) {
            if (self.0 >> i) & 1 == 1 {
                if seen {
                    f.write_str(" + ")?
                }
                f.write_str(repr)?;
                seen |= true
            }
        }

        if !seen {
            f.write_str("None")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Modifiers {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::NONE;
        for part in s.split_terminator('+').map(<str>::trim) {
            this |= match part {
                s if s.eq_ignore_ascii_case("shift") => Self::SHIFT,
                s if s.eq_ignore_ascii_case("ctrl") => Self::CTRL,
                s if s.eq_ignore_ascii_case("alt") => Self::ALT,
                modifier => return Err(format!("unknown modifier: {modifier}")),
            }
        }
        Ok(this)
    }
}

impl std::fmt::Binary for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<crossterm::event::KeyModifiers> for Modifiers {
    fn from(value: crossterm::event::KeyModifiers) -> Self {
        [
            crossterm::event::KeyModifiers::SHIFT,
            crossterm::event::KeyModifiers::CONTROL,
            crossterm::event::KeyModifiers::ALT,
        ]
        .into_iter()
        .fold(Self::NONE, |this, m| this | Self((value & m).bits()))
    }
}
