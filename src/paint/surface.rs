use crate::{
    color::Color,
    geom::{Pos2, Rect, Vec2},
};

use super::{cell::CellAttr, shape::Shape, Buffer, Cell, Renderer};

#[derive(Debug)]
pub struct Surface {
    front: Buffer,
    // we always draw to the back buffer
    back: Buffer,
}

impl Surface {
    pub fn new(size: Vec2) -> Self {
        Self {
            front: Buffer::new(size),
            back: Buffer::new(size),
        }
    }

    pub fn resize(&mut self, size: Vec2) {
        const DIRTY: Cell = Cell {
            char: '!',
            ..Cell::EMPTY
        };

        self.back.resize(size, Cell::EMPTY);
        self.front.resize(size, DIRTY);
    }

    pub const fn current(&self) -> &Buffer {
        &self.back
    }

    pub fn current_mut(&mut self) -> &mut Buffer {
        &mut self.back
    }

    pub fn crop(&mut self, rect: Rect) -> CroppedSurface {
        // TODO assert the rect is inside of our rect
        // future pos are relative to rect
        // clip outside of their rect,

        CroppedSurface {
            surface: self,
            rect,
        }
    }

    pub fn get_mut(&mut self, pos: Pos2) -> Option<&mut Cell> {
        self.current_mut().get_mut(pos)
    }

    pub fn erase(&mut self) {
        self.back.reset();
    }

    pub fn draw(&mut self, shape: impl Shape) {
        shape.draw(self.rect().size(), |pos, cell| {
            Self::put(&mut self.back, pos, cell)
        })
    }

    pub fn rect(&self) -> Rect {
        self.current().rect()
    }

    pub fn render(&mut self, renderer: &mut impl Renderer) -> std::io::Result<()> {
        let mut state = CursorState::default();
        let mut seen = false;
        let mut wrote_reset = false;

        for (pos, change) in self.front.diff(&self.back) {
            if !seen {
                renderer.begin()?;
                seen = true;
            }

            if state.maybe_move(pos) {
                renderer.move_to(pos)?;
            }

            match state.maybe_attr(change.attr) {
                Some(CellAttr::Attr(attr)) => {
                    wrote_reset = false;
                    renderer.set_attr(attr)?
                }
                Some(CellAttr::Reset) => {
                    wrote_reset = true;
                    renderer.reset_attr()?
                }
                _ => {}
            }

            match state.maybe_fg(change.fg, wrote_reset) {
                Some(Color::Rgba(fg)) => renderer.set_fg(fg)?,
                Some(Color::Reset) => renderer.reset_fg()?,
                _ => {}
            }

            match state.maybe_bg(change.bg, wrote_reset) {
                Some(Color::Rgba(bg)) => renderer.set_bg(bg)?,
                Some(Color::Reset) => renderer.reset_bg()?,
                _ => {}
            }

            wrote_reset = false;
            renderer.write(change.char)?;
        }

        if seen {
            if state.maybe_move(Pos2::ZERO) {
                renderer.move_to(Pos2::ZERO)?;
            }

            renderer.reset_bg()?;
            renderer.reset_fg()?;
            renderer.reset_attr()?;
            renderer.end()?;

            self.back.reset();
        }

        Ok(())
    }

    fn put(buffer: &mut Buffer, pos: Pos2, cell: Cell) {
        if !pos.is_normalized() || !buffer.contains(pos) {
            return;
        }

        Self::merge_cell(&mut buffer[pos], cell);
    }

    fn merge_cell(cell: &mut Cell, new_cell: Cell) {
        match (new_cell.bg, cell.bg) {
            (Color::Rgba(a), Color::Rgba(b)) => cell.bg = Color::Rgba(a.alpha_blend(b)),
            (Color::Reset | Color::Rgba(..), ..) => cell.bg = new_cell.bg,
            (Color::Reuse, _) => {}
        }

        match (new_cell.fg, cell.fg) {
            (Color::Reset | Color::Rgba(..), ..) => cell.fg = new_cell.fg,
            (Color::Reuse, _) => {}
        }

        cell.char = new_cell.char;
        cell.attr = new_cell.attr;
    }
}

pub struct CroppedSurface<'a> {
    surface: &'a mut Surface,
    rect: Rect,
}

impl<'a> CroppedSurface<'a> {
    pub const fn rect(&self) -> Rect {
        self.rect
    }

    pub fn crop<'b>(&'b mut self, rect: Rect) -> CroppedSurface<'b>
    where
        'a: 'b,
    {
        self.surface.crop(rect)
    }

    pub fn draw(&mut self, shape: impl Shape) {
        shape.draw(self.rect.size(), |pos, cell| {
            let pos = self.translate(pos);
            Surface::put(&mut self.surface.back, pos, cell)
        })
    }

    pub fn get_mut(&mut self, pos: Pos2) -> Option<&mut Cell> {
        self.surface.get_mut(pos)
    }

    fn translate(&self, pos: Pos2) -> Pos2 {
        pos + self.rect.left_top()
    }
}

#[derive(Default)]
struct CursorState {
    last: Option<Pos2>,
    fg: Option<Color>,
    bg: Option<Color>,
    attr: Option<CellAttr>,
}

impl CursorState {
    fn maybe_move(&mut self, pos: Pos2) -> bool {
        let should_move = match self.last {
            Some(last) if last.y != pos.y || last.x != pos.x - 1 => true,
            None => true,
            _ => false,
        };

        self.last = Some(pos);
        should_move
    }

    fn maybe_fg(&mut self, color: Color, resetting: bool) -> Option<Color> {
        Self::maybe_color(color, resetting, &mut self.fg)
    }

    fn maybe_bg(&mut self, color: Color, resetting: bool) -> Option<Color> {
        Self::maybe_color(color, resetting, &mut self.bg)
    }

    fn maybe_color(color: Color, resetting: bool, cache: &mut Option<Color>) -> Option<Color> {
        if matches!(color, Color::Reuse) {
            return None;
        }

        if resetting {
            cache.replace(color);
            return Some(color);
        }

        match (color, *cache) {
            (Color::Reset, None) => {
                cache.replace(color);
                Some(Color::Reset)
            }
            (Color::Reset, Some(Color::Reset)) => None,
            _ => {
                let prev = cache.replace(color);
                (prev != Some(color)).then_some(color)
            }
        }
    }

    fn maybe_attr(&mut self, attr: CellAttr) -> Option<CellAttr> {
        match (attr, self.attr) {
            (CellAttr::Reset, None) => {
                self.attr.replace(attr);
                Some(attr)
            }
            (CellAttr::Reset, Some(CellAttr::Reset)) => None,
            _ => (self.attr.replace(attr) != Some(attr)).then_some(attr),
        }
    }
}
