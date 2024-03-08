#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Attribute(u8);

impl Attribute {
    pub const RESET: Self = Self(0);
    pub const BOLD: Self = Self(1 << 0);
    pub const FAINT: Self = Self(1 << 1);
    pub const ITALIC: Self = Self(1 << 2);
    pub const UNDERLINE: Self = Self(1 << 3);
    pub const BLINK: Self = Self(1 << 4);
    pub const REVERSE: Self = Self(1 << 5);
    pub const STRIKEOUT: Self = Self(1 << 6);
}

impl Attribute {
    pub const fn is_reset(&self) -> bool {
        self.0 == 0
    }

    pub const fn is_bold(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    pub const fn is_faint(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub const fn is_italic(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub const fn is_underline(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub const fn is_blink(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    pub const fn is_reverse(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    pub const fn is_strikeout(&self) -> bool {
        self.0 & (1 << 6) != 0
    }
}

impl std::ops::BitAnd for Attribute {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitAndAssign for Attribute {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl std::ops::BitOr for Attribute {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Attribute {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl std::ops::Not for Attribute {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::fmt::Binary for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FIELDS: [&str; 7] = [
            "Bold",
            "Faint",
            "Italic",
            "Underline",
            "Blink",
            "Reverse",
            "Strikeout",
        ];

        let mut seen = false;
        for (flag, repr) in (0..).zip(FIELDS) {
            if (self.0 >> flag) & 1 == 1 {
                if seen {
                    f.write_str("| ")?;
                }
                f.write_str(repr)?;
                seen |= true
            }
        }

        if !seen {
            f.write_str("Reset")?;
        }

        Ok(())
    }
}

impl std::str::FromStr for Attribute {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::RESET;
        for part in s.split_terminator('+').map(<str>::trim) {
            this |= match part {
                s if s.eq_ignore_ascii_case("bold") => Self::BOLD,
                s if s.eq_ignore_ascii_case("faint") => Self::FAINT,
                s if s.eq_ignore_ascii_case("italic") => Self::ITALIC,
                s if s.eq_ignore_ascii_case("underline") => Self::UNDERLINE,
                s if s.eq_ignore_ascii_case("blink") => Self::BLINK,
                s if s.eq_ignore_ascii_case("reverse") => Self::REVERSE,
                s if s.eq_ignore_ascii_case("strikeout") => Self::STRIKEOUT,
                attr => return Err(format!("unknown attribute: {attr}")),
            }
        }
        Ok(this)
    }
}
