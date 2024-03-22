use super::{pos2, vec2, Align2, Pos2, Vec2};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub enum FlexFit {
    #[default]
    Loose,
    Tight,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Flow {
    #[default]
    Inline,
    Relative {
        anchor: Align2,
        offset: Dimension2,
    },
}

impl Flow {
    pub const fn is_relative(&self) -> bool {
        matches!(self, Self::Relative { .. })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum MainAxisSize {
    Max,
    Min,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

impl CrossAxisAlignment {
    pub const fn flex(&self) -> u16 {
        match self {
            Self::Start | Self::Center | Self::End => 0,
            Self::Stretch => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimension {
    pub absolute: i32,
    pub ratio: f32,
}

impl Dimension {
    pub const ZERO: Self = Self {
        absolute: 0,
        ratio: 0.0,
    };

    pub const fn absolute(absolute: i32) -> Self {
        Self {
            absolute,
            ratio: 0.0,
        }
    }

    pub const fn ratio(ratio: f32) -> Self {
        Self { absolute: 0, ratio }
    }

    pub fn resolve(&self, parent: f32) -> f32 {
        parent * self.ratio + self.absolute as f32
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimension2 {
    pub x: Dimension,
    pub y: Dimension,
}

impl Dimension2 {
    pub const ZERO: Self = Self::new(Dimension::ZERO, Dimension::ZERO);

    pub const fn new(x: Dimension, y: Dimension) -> Self {
        Self { x, y }
    }

    pub const fn absolute(x: i32, y: i32) -> Self {
        Self::new(Dimension::absolute(x), Dimension::absolute(y))
    }

    pub fn resolve(&self, parent: Size) -> Size {
        size(self.x.resolve(parent.x), self.y.resolve(parent.y))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constraints {
    pub min: Size,
    pub max: Size,
}

impl Constraints {
    #[must_use]
    pub const fn new(min: Size, max: Size) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub const fn loose(max: Size) -> Self {
        Self {
            min: Size::ZERO,
            max,
        }
    }

    #[must_use]
    pub const fn tight(value: Size) -> Self {
        Self {
            min: value,
            max: value,
        }
    }

    #[must_use]
    pub fn none() -> Self {
        Self {
            min: Size::ZERO,
            max: Size::INFINITY,
        }
    }

    #[must_use]
    pub fn constrain_min(&self, base: impl Into<Size>) -> Size {
        let base = base.into();
        base.max(self.min)
    }

    #[must_use]
    pub fn constrain(&self, base: impl Into<Size>) -> Size {
        let base = base.into();
        base.max(self.min).min(self.max)
    }

    pub fn constrain_width(&self, width: f32) -> f32 {
        width.max(self.min.x).min(self.max.x)
    }

    pub fn constrain_height(&self, height: f32) -> f32 {
        height.max(self.min.y).min(self.max.y)
    }

    pub fn is_loose(&self) -> bool {
        self.min == Size::ZERO
    }

    pub fn is_tight(&self) -> bool {
        self.min == self.max
    }

    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(
            self.min.clamp(min.min, min.max),
            self.max.clamp(max.min, max.max),
        )
    }

    #[must_use]
    pub fn size(&self) -> Size {
        if self.max.is_finite() {
            self.max
        } else if self.min.is_finite() {
            self.min
        } else {
            Size::ZERO
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    pub const ZERO: Self = size(0.0, 0.0);
    pub const INFINITY: Self = size(f32::INFINITY, f32::INFINITY);

    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn splat(d: f32) -> Self {
        Self::new(d, d)
    }

    #[must_use]
    pub fn to_pos2(&self) -> Pos2 {
        pos2(self.x.ceil() as _, self.y.ceil() as _)
    }

    #[must_use]
    pub fn to_vec2(&self) -> Vec2 {
        vec2(self.x.ceil() as _, self.y.ceil() as _)
    }

    pub fn is_infinite(&self) -> bool {
        !self.is_finite()
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.x.is_finite()
    }

    #[must_use]
    pub const fn swap(&self) -> Self {
        size(self.y, self.x)
    }

    #[must_use]
    pub fn min(&self, other: Self) -> Self {
        size(f32::min(self.x, other.x), f32::min(self.y, other.y))
    }

    #[must_use]
    pub fn max(&self, other: Self) -> Self {
        size(f32::max(self.x, other.x), f32::max(self.y, other.y))
    }

    #[must_use]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        size(
            f32::clamp(self.x, min.x, max.x),
            f32::clamp(self.y, min.y, max.y),
        )
    }
}

pub const fn size(x: f32, y: f32) -> Size {
    Size { x, y }
}

impl Default for Size {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Vec2> for Size {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Size> for Vec2 {
    fn from(value: Size) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        size(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Add<Vec2> for Size {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        self + Size::from(rhs)
    }
}

impl std::ops::Sub for Size {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        size(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Sub<Vec2> for Size {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        self - Size::from(rhs)
    }
}

impl std::ops::AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::AddAssign<Vec2> for Size {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs
    }
}

impl std::ops::SubAssign for Size {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::SubAssign<Vec2> for Size {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs
    }
}

impl std::ops::Mul<(f32, f32)> for Size {
    type Output = Self;
    fn mul(self, (x, y): (f32, f32)) -> Self::Output {
        size(self.x * x, self.y * y)
    }
}

impl std::ops::Div<(f32, f32)> for Size {
    type Output = Self;
    fn div(self, (x, y): (f32, f32)) -> Self::Output {
        size(self.x / x, self.y / y)
    }
}

impl std::ops::MulAssign<(f32, f32)> for Size {
    fn mul_assign(&mut self, rhs: (f32, f32)) {
        *self = *self * rhs
    }
}

impl std::ops::DivAssign<(f32, f32)> for Size {
    fn div_assign(&mut self, rhs: (f32, f32)) {
        *self = *self / rhs
    }
}

impl std::ops::Mul<Align2> for Size {
    type Output = Self;
    fn mul(self, rhs: Align2) -> Self::Output {
        size(self.x * rhs.x.factor(), self.y * rhs.y.factor())
    }
}
