#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[must_use]
    pub fn as_array_mut(&mut self) -> [&mut f32; 3] {
        [&mut self.x, &mut self.y, &mut self.z]
    }

    #[must_use]
    pub fn cos(&self) -> Self {
        vec3(self.x.cos(), self.y.cos(), self.z.cos())
    }

    #[must_use]
    pub fn sin(&self) -> Self {
        vec3(self.x.sin(), self.y.sin(), self.z.sin())
    }

    #[must_use]
    pub fn sin_cos(&self) -> (Self, Self) {
        let (sx, cx) = self.x.sin_cos();
        let (sy, cy) = self.y.sin_cos();
        let (sz, cz) = self.z.sin_cos();
        (vec3(sx, sy, sz), vec3(cx, cy, cz))
    }

    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        vec3(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        vec3(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
