use crate::{
    geom::{Pos2, Vec2},
    paint::{Cell, Label, Styled},
};

use super::shape::Shape;

pub struct MappedStyle<T, F>
where
    T: Label,
    F: Fn(Pos2, Cell) -> Cell,
{
    pub label: Styled<T>,
    map: F,
}

impl<T: Label, F: Fn(Pos2, Cell) -> Cell> std::fmt::Debug for MappedStyle<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.label.fmt(f)
    }
}

impl<T: Label> MappedStyle<T, fn(Pos2, Cell) -> Cell> {
    pub fn new(label: impl Into<Styled<T>>) -> Self {
        Self {
            label: label.into(),
            map: styled_identity,
        }
    }

    pub fn map<F>(self, map: F) -> MappedStyle<T, F>
    where
        F: Fn(Pos2, Cell) -> Cell,
    {
        MappedStyle {
            label: self.label,
            map,
        }
    }
}

impl<T: Label, F: Fn(Pos2, Cell) -> Cell> MappedStyle<T, F> {
    pub fn into_static(self) -> MappedStyle<T::Static, F> {
        MappedStyle {
            label: self.label.into_static(),
            map: self.map,
        }
    }
}

pub fn styled_identity(_: Pos2, cell: Cell) -> Cell {
    cell
}

impl<T, F> Shape for MappedStyle<T, F>
where
    T: Label,
    F: Fn(Pos2, Cell) -> Cell,
{
    fn draw(&self, size: Vec2, mut put: impl FnMut(Pos2, Cell)) {
        self.label.draw(size, |pos, cell| {
            let cell = (self.map)(pos, cell);
            put(pos, cell)
        });
    }
}
