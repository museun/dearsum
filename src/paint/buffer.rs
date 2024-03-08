use crate::{
    color::Color,
    geom::{pos2, rect, Pos2, Rect, Vec2},
};

use super::Cell;

#[derive(Debug)]
pub struct Buffer {
    pub(crate) cells: Vec<Cell>,
    size: Vec2,
}

impl Buffer {
    pub fn new(size: Vec2) -> Self {
        Self {
            cells: vec![Cell::EMPTY; size.x as usize * size.y as usize],
            size,
        }
    }

    pub fn resize(&mut self, size: Vec2, cell: Cell) {
        if self.size == size {
            for old in &mut self.cells {
                *old = cell;
            }
            return;
        }

        *self = Self {
            cells: vec![cell; size.x as usize * size.y as usize],
            size,
        }
    }

    pub fn reset(&mut self) {
        for x in &mut self.cells {
            *x = Cell::EMPTY;
        }
    }

    pub const fn rect(&self) -> Rect {
        rect(self.size)
    }

    pub const fn contains(&self, pos: Pos2) -> bool {
        pos.is_normalized() && pos.x < self.size.x && pos.y < self.size.y
    }

    pub fn get(&self, pos: Pos2) -> Option<&Cell> {
        if !pos.is_normalized() {
            return None;
        }
        self.cells.get(Self::pos_to_index(pos, self.size.x))
    }

    pub fn get_mut(&mut self, pos: Pos2) -> Option<&mut Cell> {
        if !pos.is_normalized() {
            return None;
        }
        self.cells.get_mut(Self::pos_to_index(pos, self.size.x))
    }

    pub fn diff<'a>(&'a mut self, other: &'a Self) -> impl Iterator<Item = (Pos2, &'a Cell)> {
        self.cells
            .iter_mut()
            .zip(other.cells.iter())
            .enumerate()
            .filter_map(|(i, (left, right))| {
                if *left == *right || (right.fg == Color::Reuse && right.bg == Color::Reuse) {
                    return None;
                }

                *left = *right;
                Some((Self::index_to_pos(i, self.size.x), right))
            })
    }

    const fn pos_to_index(pos: Pos2, w: i32) -> usize {
        assert!(pos.is_normalized());
        (pos.y * w + pos.x) as usize
    }

    const fn index_to_pos(index: usize, w: i32) -> Pos2 {
        let index = index as i32;
        pos2(index % w, index / w)
    }
}

impl std::ops::Index<Pos2> for Buffer {
    type Output = Cell;
    fn index(&self, index: Pos2) -> &Self::Output {
        &self.cells[Self::pos_to_index(index, self.size.x)]
    }
}

impl std::ops::IndexMut<Pos2> for Buffer {
    fn index_mut(&mut self, index: Pos2) -> &mut Self::Output {
        &mut self.cells[Self::pos_to_index(index, self.size.x)]
    }
}
