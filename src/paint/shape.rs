use crate::geom::{pos2, Pos2, Vec2};

use super::Cell;

pub trait Shape {
    fn draw(&self, size: Vec2, put: impl FnMut(Pos2, Cell));
}

impl<T: Shape> Shape for &T {
    fn draw(&self, size: Vec2, put: impl FnMut(Pos2, Cell)) {
        <T as Shape>::draw(self, size, put)
    }
}

mod filled;
pub use filled::Filled;

mod border;
pub use border::Border;

mod line;
pub use line::{HorizontalLine, VerticalLine};

impl Shape for Cell {
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        for y in 0..size.y.max(1) {
            for x in 0..size.x.max(1) {
                put(pos2(x, y), *self)
            }
        }
    }
}
