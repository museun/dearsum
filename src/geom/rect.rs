use super::{math::lerp, pos2, vec2, Pos2, Vec2};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    pub min: Pos2,
    pub max: Pos2,
}

#[must_use]
pub const fn rect(size: Vec2) -> Rect {
    Rect::from_min_size(Pos2::ZERO, size)
}

impl Rect {
    pub const ZERO: Self = rect(Vec2::ZERO);

    #[must_use]
    pub const fn from_min_max(min: Pos2, max: Pos2) -> Self {
        // TODO this should normalize the positions
        Self { min, max }
    }

    #[must_use]
    pub const fn from_min_size(min: Pos2, size: Vec2) -> Self {
        Self {
            min,
            max: pos2(
                min.x.saturating_add_unsigned(size.x as u32),
                min.y.saturating_add_unsigned(size.y as u32),
            ),
        }
    }

    #[must_use]
    pub fn from_center_size(center: Pos2, size: Vec2) -> Self {
        Self {
            min: center - (size / 2),
            max: center + (size / 2),
        }
    }

    // TODO this may overflow
    pub const fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn size(&self) -> Vec2 {
        (self.max - self.min).to_vec2()
    }

    #[must_use]
    pub fn clip(&self, size: Vec2) -> Self {
        Self::from_min_size(self.min.normalize(), size.min(self.size()))
    }

    #[must_use]
    pub fn clamp(&self, pos: Pos2) -> Pos2 {
        pos.clamp(self.min, self.max)
    }

    #[must_use]
    pub fn clamp_rect(&self, other: Self) -> Self {
        let min = other.min.max(self.min).min(pos2(
            self.right().saturating_sub(other.width()),
            self.bottom().saturating_sub(other.height()),
        ));
        Self::from_min_size(min, other.size())
    }

    pub fn distance_to_point(&self, pos: Pos2) -> i32 {
        (self.distance_sq_to_point(pos) as f32).sqrt() as _
    }

    pub fn distance_sq_to_point(&self, pos: Pos2) -> i32 {
        fn distance(min: i32, max: i32, t: i32) -> i32 {
            match () {
                _ if min > t => min - t,
                _ if t > max => t - max,
                _ => 0,
            }
        }

        let dx = distance(self.min.x, self.max.x, pos.x);
        let dy = distance(self.min.y, self.max.y, pos.y);
        pos2(dx, dy).length_sq()
    }

    pub fn contains(&self, pos: Pos2) -> bool {
        self.min.x <= pos.x && pos.x < self.max.x && self.min.y <= pos.y && pos.y < self.max.y
    }

    pub fn contains_rect(&self, other: Self) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    #[must_use]
    pub fn shrink2(&self, d: Vec2) -> Self {
        Self::from_min_max(self.min + d, self.max - d)
    }

    #[must_use]
    pub fn shrink(&self, d: i32) -> Self {
        self.shrink2(Vec2::splat(d))
    }

    #[must_use]
    pub fn expand2(&self, d: Vec2) -> Self {
        Self::from_min_max(self.min - d, self.max + d)
    }

    #[must_use]
    pub fn expand(&self, d: i32) -> Self {
        self.expand2(Vec2::splat(d))
    }

    #[must_use]
    pub fn translate(&self, vec: Vec2) -> Self {
        Self::from_min_size(self.min + vec, self.size())
    }

    #[must_use]
    pub fn intersection(&self, other: Self) -> Self {
        Self::from_min_max(self.min.max(other.min), self.max.min(other.max))
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.min.x <= other.max.x
            && other.min.x <= self.max.x
            && self.min.y <= other.min.y
            && other.max.y <= self.max.y
    }

    #[must_use]
    pub fn union(&self, other: Self) -> Self {
        Self::from_min_max(self.min.min(other.min), self.max.max(other.max))
    }

    pub const fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub const fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    pub const fn left(&self) -> i32 {
        self.min.x
    }

    pub const fn right(&self) -> i32 {
        self.max.x.saturating_sub_unsigned(1)
    }

    pub const fn top(&self) -> i32 {
        self.min.y
    }

    pub const fn bottom(&self) -> i32 {
        self.max.y.saturating_sub_unsigned(1)
    }

    pub fn center(&self) -> Pos2 {
        pos2((self.min.x + self.max.x) / 2, (self.min.y + self.max.y) / 2)
    }

    pub const fn left_top(&self) -> Pos2 {
        pos2(self.left(), self.top())
    }

    pub const fn right_top(&self) -> Pos2 {
        pos2(self.right(), self.top())
    }

    pub const fn right_bottom(&self) -> Pos2 {
        pos2(self.right(), self.bottom())
    }

    pub const fn left_bottom(&self) -> Pos2 {
        pos2(self.left(), self.bottom())
    }

    #[must_use]
    pub fn split_horizontal_ratio(self, spacing: i32, ratio: f32) -> (Rect, Rect) {
        let p = lerp(self.min.x as f32, self.max.x as f32, ratio) as i32;
        let left = Rect::from_min_max(self.min, pos2(p - spacing, self.max.y));
        let right = Rect::from_min_max(pos2(p, self.min.y), self.max);
        (left, right)
    }

    #[must_use]
    pub fn split_vertical_ratio(self, spacing: i32, ratio: f32) -> (Rect, Rect) {
        let p = lerp(self.min.y as f32, self.max.y as f32, ratio) as i32;
        let top = Rect::from_min_max(self.min, pos2(self.max.x, p - spacing));
        let bottom = Rect::from_min_max(pos2(self.min.x, p), self.max);
        (top, bottom)
    }

    #[must_use]
    pub fn split_n_vertical<const N: usize>(self) -> [Rect; N] {
        let mut out = [Rect::ZERO; N];
        let p = (self.height() as usize).div_ceil(N) as i32;
        let mut cursor = self.left_top();

        for temp in &mut out {
            *temp = Rect::from_min_size(cursor, vec2(self.width(), p));
            cursor.y = temp.bottom() + 1;
        }

        let last = &mut out[N - 1];
        *last = Rect::from_min_max(last.min, self.max);

        out
    }

    #[must_use]
    pub fn split_n_horizontal<const N: usize>(self) -> [Rect; N] {
        let mut out = [Rect::ZERO; N];
        let p = (self.width() as usize).div_ceil(N) as i32;
        let mut cursor = self.left_top();

        for temp in &mut out {
            *temp = Rect::from_min_size(cursor, vec2(p, self.bottom()));
            cursor.x = temp.right() + 1;
        }

        let last = &mut out[N - 1];
        *last = Rect::from_min_max(last.min, self.max);

        out
    }

    /// Clockwise from the left-top
    ///
    /// left top -> right top -> right bottom -> left bottom
    pub const fn corners(&self) -> [Pos2; 4] {
        [
            self.left_top(),
            self.right_top(),
            self.right_bottom(),
            self.left_bottom(),
        ]
    }
}

impl std::ops::Add<Pos2> for Rect {
    type Output = Self;
    fn add(self, rhs: Pos2) -> Self::Output {
        self.translate(rhs.to_vec2())
    }
}
impl std::ops::AddAssign<Pos2> for Rect {
    fn add_assign(&mut self, rhs: Pos2) {
        *self = *self + rhs
    }
}

impl std::ops::Sub<Pos2> for Rect {
    type Output = Self;
    fn sub(self, rhs: Pos2) -> Self::Output {
        self.translate(-rhs.to_vec2())
    }
}

impl std::ops::SubAssign<Pos2> for Rect {
    fn sub_assign(&mut self, rhs: Pos2) {
        *self = *self - rhs
    }
}

impl std::ops::Add<Vec2> for Rect {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Rect::from_min_size(self.min, self.size() + rhs)
    }
}
impl std::ops::AddAssign<Vec2> for Rect {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs
    }
}

impl std::ops::Sub<Vec2> for Rect {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Rect::from_min_size(self.min, self.size() - rhs)
    }
}

impl std::ops::SubAssign<Vec2> for Rect {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs
    }
}
