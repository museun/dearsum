use crate::{
    geom::{pos2, Pos2, Vec2},
    paint::Cell,
};

use super::Shape;

pub struct HorizontalLine {
    cell: Cell,
}

impl Default for HorizontalLine {
    fn default() -> Self {
        Self::new()
    }
}

impl HorizontalLine {
    pub fn new() -> Self {
        Self::cell('─')
    }

    pub fn cell(cell: impl Into<Cell>) -> Self {
        Self { cell: cell.into() }
    }
}

impl Shape for HorizontalLine {
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        for x in 0..=size.x {
            put(pos2(x, 0), self.cell)
        }
    }
}

pub struct VerticalLine {
    cell: Cell,
}

impl Default for VerticalLine {
    fn default() -> Self {
        Self::new()
    }
}

impl VerticalLine {
    pub fn new() -> Self {
        Self::cell('│')
    }

    pub fn cell(cell: impl Into<Cell>) -> Self {
        Self { cell: cell.into() }
    }
}

impl Shape for VerticalLine {
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        for y in 0..=size.y {
            put(pos2(0, y), self.cell)
        }
    }
}
