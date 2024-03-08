use super::Rgba;
use crate::geom::math::almost_eq;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsl(pub f32, pub f32, pub f32);

impl Hsl {
    pub const fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self(hue, saturation, lightness)
    }

    pub const fn hue(&self) -> f32 {
        self.0
    }

    pub const fn saturation(&self) -> f32 {
        self.1
    }

    pub const fn lightness(&self) -> f32 {
        self.2
    }

    #[must_use]
    pub fn darken(&self, ratio: f32) -> Self {
        let Self(h, s, mut l) = *self;
        l = (l - ratio).clamp(0.0, 1.0);
        Self(h, s, l)
    }

    #[must_use]
    pub fn lighten(&self, ratio: f32) -> Self {
        let Self(h, s, mut l) = *self;
        l = (l + ratio).clamp(0.0, 1.0);
        Self(h, s, l)
    }

    #[must_use]
    pub fn complement(&self) -> Self {
        let Self(h, mut s, l) = *self;
        s = (s + 180.0) % 360.0;
        Self(h, s, l)
    }

    #[must_use]
    pub fn mix(&self, left: f32, other: Self, right: f32) -> Self {
        let &Self(h1, s1, l1) = self;
        let Self(h2, s2, l2) = other;

        let h = if (h1 - h2).abs() > 180.0 {
            let (a, b) = if h1 < h2 {
                (h1 + 360.0, h2)
            } else {
                (h1, h2 + 360.0)
            };
            left.mul_add(a, right * b) / (left + right)
        } else {
            left.mul_add(h1, right * h2) / (left + right)
        };

        let s = left.mul_add(s1, right * s2) / (left + right);
        let l = left.mul_add(l1, right * l2) / (left + right);
        Self(h, s, l)
    }
}

impl From<Rgba> for Hsl {
    fn from(value: Rgba) -> Self {
        let [r, g, b, _] = value.as_float();
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);

        let l = 0.5 * (max + min);
        if almost_eq(min, max) {
            return Self(0.0, 0.0, l);
        }

        let diff = max - min;
        let h = match () {
            _ if almost_eq(max, r) => 60.0 * (g - b) / diff,
            _ if almost_eq(max, g) => 60.0 * (b - r) / diff + 120.0,
            _ if almost_eq(max, b) => 60.0 * (r - g) / diff + 240.0,
            _ => 0.0,
        };

        let h = (h + 360.0) % 360.0;
        let s = if 0.0 < l && l <= 0.5 {
            diff / (2.0 * l)
        } else {
            diff / (1.0 - (2.0 * l - 1.0).abs())
        };

        Self(h, s, l)
    }
}

impl From<&Rgba> for Hsl {
    fn from(value: &Rgba) -> Self {
        (*value).into()
    }
}
