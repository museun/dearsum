use super::{pos2, size, Pos2, Size};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[must_use]
pub const fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    pub const ZERO: Self = vec2(0, 0);

    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn splat(d: i32) -> Self {
        Self::new(d, d)
    }

    #[must_use]
    pub fn min(&self, other: Self) -> Self {
        self.to_pos2().min(other.to_pos2()).to_vec2()
    }

    #[must_use]
    pub fn max(&self, other: Self) -> Self {
        self.to_pos2().max(other.to_pos2()).to_vec2()
    }

    #[must_use]
    pub const fn swap(self) -> Self {
        vec2(self.y, self.x)
    }

    #[must_use]
    pub const fn to_pos2(self) -> Pos2 {
        pos2(self.x, self.y)
    }

    pub fn length(&self) -> i32 {
        (self.x as f32).hypot(self.y as f32) as i32
    }

    pub const fn length_sq(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub const fn to_size(&self) -> Size {
        size(self.x as f32, self.y as f32)
    }

    // pub fn from_angle(&self) -> i32{}
    // pub fn rotate(&self) -> i32{}
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Add<i32> for Vec2 {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        vec2(self.x + rhs, self.y + rhs)
    }
}

impl std::ops::Sub<i32> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        vec2(self.x - rhs, self.y - rhs)
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<i32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        vec2(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec2((self.x as f32 * rhs) as i32, (self.y as f32 * rhs) as i32)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        vec2((self.x as f32 / rhs) as i32, (self.y as f32 / rhs) as i32)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        vec2((self * rhs.x as f32) as i32, (self * rhs.y as f32) as i32)
    }
}

impl std::ops::Div<Vec2> for f32 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        vec2((self / rhs.x as f32) as i32, (self / rhs.y as f32) as i32)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        vec2(-self.x, -self.y)
    }
}

// TODO rotation
