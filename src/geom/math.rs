use std::ops::{Add, Div, Mul, Sub};

pub fn almost_eq(left: f32, right: f32) -> bool {
    if left == right {
        return true;
    }
    let abs = left.abs().max(right.abs());
    abs <= f32::EPSILON || ((left - right).abs() / abs) <= f32::EPSILON
}

pub fn lerp<T: Num>(x: T, y: T, t: T) -> T {
    (T::ONE - t) * x + t * y
}

pub fn inverse_lerp<T: Num>(x: T, y: T, t: T) -> Option<T> {
    if x == y {
        return None;
    }
    Some((t - x) / (y - x))
}

pub fn remap<T, U>(d: T, (from_x, from_y): (T, T), (to_x, to_y): (U, U)) -> U
where
    T: Num + Cast<U> + Cast,
    U: Num + Cast<T> + Cast,
{
    let t = (d - from_x) / (from_y - from_x);
    lerp(to_x.cast(), to_y.cast(), t).cast()
}

pub fn remap_clamp<T, U>(d: T, (from_x, from_y): (T, T), (to_x, to_y): (U, U)) -> U
where
    T: Num + Cast<U> + Cast,
    U: Num + Cast<T> + Cast,
{
    if from_y < from_x {
        return remap_clamp(d, (from_y, from_x), (to_x, to_y));
    }

    if d <= from_x {
        return to_x;
    }
    if from_y <= d {
        return to_y;
    }

    let t = (d - from_x) / (from_y - from_x);
    if T::ONE <= t {
        return to_y;
    }

    lerp(to_x.cast(), to_y.cast(), t).cast()
}

pub trait Cast<T: Num = Self>: Num {
    fn cast(self) -> T;
}

impl<T> Cast<T> for T
where
    T: Num,
{
    fn cast(self) -> T {
        self
    }
}

impl Cast<i32> for f32 {
    fn cast(self) -> i32 {
        self.ceil() as _
    }
}

impl Cast<f32> for i32 {
    fn cast(self) -> f32 {
        self as _
    }
}

pub trait Num:
    PartialOrd
    + std::fmt::Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
{
    const ZERO: Self;
    const ONE: Self;
}

impl Num for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Num for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
