use super::{math::lerp, vec2, Vec2};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos2 {
    pub x: i32,
    pub y: i32,
}

#[must_use]
pub const fn pos2(x: i32, y: i32) -> Pos2 {
    Pos2 { x, y }
}

impl Pos2 {
    pub const ZERO: Self = pos2(0, 0);

    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn splat(d: i32) -> Self {
        Self::new(d, d)
    }

    pub const fn is_normalized(&self) -> bool {
        (self.x == 0 || self.y == 0) || (self.x.is_positive() && self.x.is_positive())
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        pos2(self.x.max(0), self.y.max(0))
    }

    #[must_use]
    pub fn min(&self, other: Self) -> Self {
        pos2(self.x.min(other.x), self.y.min(other.y))
    }

    #[must_use]
    pub fn max(&self, other: Self) -> Self {
        pos2(self.x.max(other.x), self.y.max(other.y))
    }

    #[must_use]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        pos2(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    #[must_use]
    pub const fn swap(self) -> Self {
        pos2(self.y, self.x)
    }

    #[must_use]
    pub const fn to_vec2(self) -> Vec2 {
        vec2(self.x, self.y)
    }

    pub fn distance(self, other: Self) -> i32 {
        let (x0, x1) = (self.x as f32, other.x as f32);
        let (y0, y1) = (self.y as f32, other.y as f32);
        (x0 - x1).hypot(y0 - y1) as i32
    }

    pub fn distance_sq(self, other: Self) -> i32 {
        let x = self.x as f32 - other.x as f32;
        let y = self.y as f32 - other.y as f32;
        x.mul_add(x, y * y) as i32
    }

    pub fn length(&self) -> i32 {
        (self.x as f32).hypot(self.y as f32) as i32
    }

    pub const fn length_sq(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn lerp(self, other: Self, t: i32) -> Self {
        pos2(lerp(self.x, other.x, t), lerp(self.y, self.y, t))
    }
}

impl std::ops::Add<Vec2> for Pos2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        pos2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Vec2> for Pos2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        pos2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add for Pos2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        pos2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Pos2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        pos2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::AddAssign for Pos2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Pos2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::AddAssign<Vec2> for Pos2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Vec2> for Pos2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}
