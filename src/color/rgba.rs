use std::f32::consts::PI;

use super::Hsl;
use crate::geom::math::{almost_eq, inverse_lerp};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Default for Rgba {
    fn default() -> Self {
        Self::OPAQUE
    }
}

impl Rgba {
    pub const TRANSPARENT: Self = Self(0, 0, 0, 0);
    pub const OPAQUE: Self = Self(255, 255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b, 255)
    }

    pub const fn from_u16(color: u16) -> Self {
        let a = (color >> 12) & ((1 << 4) - 1);
        let is_16 = a == 0;
        let offset = if is_16 { 4 } else { 0 };

        let r = ((color >> (12 - offset)) & 0xF) as u8;
        let g = ((color >> (8 - offset)) & 0xF) as u8;
        let b = ((color >> (4 - offset)) & 0xF) as u8;
        let a = if is_16 { 0xF } else { (color & 0xF) as u8 };

        Self((r << 4) | r, (g << 4) | g, (b << 4) | b, (a << 4) | a)
    }

    pub const fn from_u32(color: u32) -> Self {
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = ((color) & 0xFF) as u8;
        Self(r, g, b, 0xFF)
    }

    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.3 = alpha;
        self
    }

    #[track_caller]
    pub const fn from_static(color: &'static str) -> Self {
        #[track_caller]
        const fn to_digit(d: u8) -> u8 {
            assert!(d.is_ascii_hexdigit(), "invalid hex digit");
            match d.wrapping_sub(b'0') {
                d if d < 10 => d,
                _ => d.to_ascii_lowercase().saturating_sub(b'a') + 10,
            }
        }

        #[track_caller]
        const fn pack(high: u8, low: u8) -> u8 {
            to_digit(high) << 4 | to_digit(low)
        }

        let color = color.as_bytes();
        let len = color.len();
        let mut start = 0;

        while matches!(color[start], b' ' | b'\t' | b'\n') {
            start += 1;
        }

        let mut end = start;
        while end < color.len() && !matches!(color[end], b' ' | b'\t' | b'\n') {
            end += 1;
        }

        let (_, mut color) = color.split_at(start);
        if end - start < len {
            (color, _) = color.split_at(end - start);
        }

        let ((rh, gh, bh, ah), (rl, gl, bl, al)) = match color {
            &[b'#', rh, rl, gh, gl, bh, bl] => ((rh, gh, bh, b'F'), (rl, gl, bl, b'F')),
            &[b'#', rh, rl, gh, gl, bh, bl, ah, al] => ((rh, gh, bh, ah), (rl, gl, bl, al)),

            &[b'#', r, g, b] => ((r, g, b, b'F'), (r, g, b, b'F')),
            &[b'#', r, g, b, a] => ((r, g, b, a), (r, g, b, a)),

            [a, d @ ..] if !matches!(a, b'#') && matches!(d.len(), 7 | 5 | 3 | 2) => {
                panic!("missing '#' prefix")
            }
            &[] => panic!("empty string"),
            _ => panic!("invalid color. syntax: #RRGGBB | #RRGGBBAA | #RGB | #RGBA"),
        };

        Self(pack(rh, rl), pack(gh, gl), pack(bh, bl), pack(ah, al))
    }

    pub const fn red(&self) -> u8 {
        self.0
    }

    pub const fn green(&self) -> u8 {
        self.1
    }

    pub const fn blue(&self) -> u8 {
        self.2
    }

    pub const fn alpha(&self) -> u8 {
        self.3
    }

    pub const fn opaque(mut self) -> Self {
        self.3 = 255;
        self
    }

    pub fn transparent(mut self, alpha: f32) -> Self {
        let t = inverse_lerp(0.0, 100.0, alpha).unwrap_or(1.0);
        self.3 = (t * 255.0) as u8;
        self
    }

    pub fn as_float(&self) -> [f32; 4] {
        let Self(r, g, b, a) = *self;
        let scale = |d| (d as f32 / 256.0);
        [scale(r), scale(g), scale(b), scale(a)]
    }

    pub fn from_float([r, g, b, a]: [f32; 4]) -> Self {
        let scale = |d: f32| {
            assert!(d.is_finite());
            (255.0_f32 * d).round() as u8
        };

        assert!(r.is_finite(), "r");
        assert!(g.is_finite(), "g");
        assert!(b.is_finite(), "b");
        assert!(a.is_finite(), "a");

        Self(scale(r), scale(g), scale(b), scale(a))
    }

    pub fn mix(self, left: f32, other: Self, right: f32) -> Self {
        let [r1, g1, b1, a1] = self.as_float();
        let [r2, g2, b2, a2] = other.as_float();

        let ratio = left + right;
        Self::from_float([
            left.mul_add(r1, right * r2) / ratio,
            left.mul_add(g1, right * g2) / ratio,
            left.mul_add(b1, right * b2) / ratio,
            a1.max(a2),
        ])
    }

    pub fn blend(self, other: Self, mix: f32) -> Self {
        self.mix(mix, other, mix)
    }

    pub fn alpha_blend(self, other: Self) -> Self {
        if self.alpha() == 0 {
            return other;
        }
        if self.alpha() == 255 {
            return self;
        }

        fn blend(a: i32, l: u8, r: u8) -> u8 {
            ((a * l as i32 + (255 - a) * r as i32) / 255) as u8
        }

        let a = self.alpha() as i32;
        let r = blend(a, self.red(), other.red());
        let g = blend(a, self.green(), other.green());
        let b = blend(a, self.blue(), other.blue());
        Self(r, g, b, 255)
    }

    pub fn flat_blend(self, other: Self, mix: f32) -> Self {
        let [r1, g1, b1, a1] = self.as_float();
        let [r2, g2, b2, a2] = other.as_float();
        Self::from_float([
            (r2 - r1).mul_add(mix, r1),
            (g2 - g1).mul_add(mix, g1),
            (b2 - b1).mul_add(mix, b1),
            a1.max(a2),
        ])
    }

    pub fn linear_blend(self, other: Self) -> Self {
        let this = Hsl::from(self);
        let other = Hsl::from(other);
        this.mix(0.5, other, 0.5).into()
    }

    pub fn complement(&self) -> Self {
        Hsl::from(self).complement().into()
    }

    pub fn rotate(&self, angle: f32) -> Self {
        let rot = angle.to_radians();
        let (sin, cos) = rot.sin_cos();
        let [r, g, b, a] = self.as_float();
        let r = r * cos - g * sin;
        let g = r * sin + g * cos;
        Self::from_float([r, g, b, a])
    }

    pub fn darken(self, ratio: f32) -> Self {
        Hsl::from(self).darken(ratio).into()
    }

    pub fn lighten(self, ratio: f32) -> Self {
        Hsl::from(self).lighten(ratio).into()
    }

    pub fn sine_color(n: f32) -> Self {
        let h = n * ((1.0 + 5.0_f32.sqrt()) / 2.0);
        let h = (h + 0.5) * -1.0;
        let r = (PI * h).sin();
        let g = (PI * (h + 0.3)).sin();
        let b = (PI * (h + 0.6)).sin();
        Self::from_float([r * r, g * g, b * b, 1.0])
    }
}

impl From<u32> for Rgba {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl std::fmt::Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b, a) = self;
        if self.alpha() == 0 {
            write!(f, "rgb({r}, {g}, {b})")
        } else {
            write!(f, "rgb({r}, {g}, {b}, {a})")
        }
    }
}

impl std::fmt::LowerHex for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b, a) = self;
        write!(f, "0x{r:02x}{g:02x}{b:02x}",)?;
        if self.alpha() != 0xFF {
            write!(f, "{a:02x}")?;
        }
        Ok(())
    }
}

impl std::fmt::UpperHex for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b, a) = self;
        write!(f, "0x{r:02X}{g:02X}{b:02X}",)?;
        if self.alpha() != 0xFF {
            write!(f, "{a:02X}")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Rgba {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: & str = "rgba must be in the form of rgb(r, g, b) or rgb(r, g, b, a) or #rrggbb or #rrggbba or #rgb or #rgba";
        if let Some(s) = s.strip_prefix('#') {
            return match s.len() {
                3 | 4 => u16::from_str_radix(s, 16)
                    .map_err(|_| "invalid hex digits")
                    .map(Self::from_u16),
                6 | 8 => u32::from_str_radix(s, 16)
                    .map_err(|_| "invalid hex digits")
                    .map(Self::from_u32),
                _ => Err(ERR),
            };
        }

        if s.starts_with("rgb(") && s.ends_with(')') {
            let s = &s[4..s.len() - 1];
            let mut iter = s.split_terminator(',').map(|s| s.trim().parse());
            let r = iter
                .next()
                .and_then(Result::ok)
                .ok_or("invalid red channel")?;

            let g = iter
                .next()
                .and_then(Result::ok)
                .ok_or("invalid green channel")?;

            let b = iter
                .next()
                .and_then(Result::ok)
                .ok_or("invalid blue channel")?;

            let a = match iter.next() {
                None => 0xFF,
                Some(Ok(a)) => a,
                Some(Err(..)) => return Err("invalid alpha channel"),
            };

            return Ok(Self(r, g, b, a));
        }

        Err(ERR)
    }
}

impl From<Hsl> for Rgba {
    fn from(value: Hsl) -> Self {
        let Hsl(mut h, s, l) = value;
        if almost_eq(s, 0.0) {
            return Self::from_float([l, l, l, 1.0]);
        }

        h /= 360.0;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            s.mul_add(-l, l + s)
        };
        let p = 2.0_f32.mul_add(l, -q);

        let r = hue(p, q, h + (1.0_f32 / 3.0));
        let g = hue(p, q, h);
        let b = hue(p, q, h - (1.0_f32 / 3.0));

        Self::from_float([r, g, b, 1.0])
    }
}

fn hue(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if 6.0 * t < 1.0 {
        // TODO this is wrong somehow
        ((p + (q - p)) * 6.0 * t).clamp(0.0, 1.0)
    } else if 2.0 * t < 1.0 {
        q
    } else if 3.0 * t < 2.0 {
        ((q - p) * ((2.0 / 3.0) - t))
            .mul_add(6.0, p)
            .clamp(0.0, 1.0)
    } else {
        p
    }
}

impl From<&Hsl> for Rgba {
    fn from(value: &Hsl) -> Self {
        (*value).into()
    }
}
