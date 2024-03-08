use crate::{
    geom::{pos2, Pos2, Vec2},
    paint::Cell,
};

use super::Shape;

pub struct Line<F: Fn(Pos2) -> Cell> {
    inner: LineDir,
    put: F,
}

// FIXME: why do any of these have a length?
enum LineDir {
    Horizontal(i32),
    Vertical(i32),
    Custom(Pos2, Pos2),
}

impl Line<fn(Pos2) -> Cell> {
    pub const fn horizontal(w: i32) -> Self {
        Self {
            inner: LineDir::Horizontal(w),
            put: |_| Cell::new('─'),
        }
    }

    pub const fn vertical(h: i32) -> Self {
        Self {
            inner: LineDir::Vertical(h),
            put: |_| Cell::new('─'),
        }
    }

    pub const fn new<F: Fn(Pos2) -> Cell>(start: Pos2, end: Pos2, put: F) -> Line<F> {
        Line {
            inner: LineDir::Custom(start, end),
            put,
        }
    }
}

impl Line<fn(Pos2) -> Cell> {
    pub fn custom_cell<F: Fn(Pos2) -> Cell>(self, put: F) -> Line<F> {
        Line {
            inner: self.inner,
            put,
        }
    }
}

impl<F: Fn(Pos2) -> Cell> Shape for Line<F> {
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        match self.inner {
            LineDir::Horizontal(w) => {
                for x in 0..size.x.min(w) {
                    let pos = pos2(x, 0);
                    let cell = (self.put)(pos);
                    put(pos, cell)
                }
            }
            LineDir::Vertical(h) => {
                for y in 0..size.y.min(h) {
                    let pos = pos2(0, y);
                    let cell = (self.put)(pos);
                    put(pos, cell)
                }
            }
            LineDir::Custom(start, end) => {
                for y in start.y..=size.y.min(end.y) {
                    for x in start.x..=size.x.min(end.x) {
                        let pos = pos2(x, y);
                        let cell = (self.put)(pos);
                        put(pos, cell)
                    }
                }
            }
        }
    }
}
