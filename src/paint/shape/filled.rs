use crate::{
    color::Rgba,
    geom::{pos2, Pos2, Vec2},
    paint::Cell,
};

use super::Shape;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Filled {
    cell: Cell,
}

impl Filled {
    pub fn new(cell: impl Into<Cell>) -> Self {
        Self { cell: cell.into() }
    }

    pub fn bg(bg: impl Into<Rgba>) -> Self {
        Self::new(Cell::new(' ').bg(bg.into()))
    }
}

impl Shape for Filled {
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        for y in 0..size.y {
            for x in 0..size.x {
                put(pos2(x, y), self.cell)
            }
        }
    }
}
