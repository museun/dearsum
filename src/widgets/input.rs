use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    context::{EventCtx, LayoutCtx, PaintCtx},
    geom::{math::remap, pos2, size, Constraints, Size},
    input::{Event, Handled, Interest, Key},
    paint::{shape::Filled, Attribute, Styled},
    widget::Response,
    Widget, WidgetExt as _,
};

#[derive(Clone, Debug, Default)]
pub struct InputBuffer {
    inner: Rc<RefCell<Inner>>,
}

impl InputBuffer {
    pub fn new(buffer: impl ToString) -> Self {
        let buffer = buffer.to_string();
        Self {
            inner: Rc::new(RefCell::new(Inner {
                cursor: buffer.len(),
                buffer,
            })),
        }
    }

    pub fn set(&self, data: impl ToString) {
        let buffer = data.to_string();
        let data = Inner {
            cursor: buffer.len(),
            buffer,
        };
        let _ = std::mem::replace(&mut *self.inner.borrow_mut(), data);
    }

    pub fn clear(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.buffer.clear();
        inner.cursor = 0;
    }

    pub fn len(&self) -> usize {
        self.inner.borrow().buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_str(&self) -> Ref<'_, str> {
        let buf = self.inner.borrow();
        Ref::map(buf, |f| &*f.buffer)
    }
}

impl InputBuffer {
    fn take(&mut self) -> String {
        let mut inner = self.inner.borrow_mut();
        inner.cursor = 0;
        std::mem::take(&mut inner.buffer)
    }

    fn insert(&mut self, ch: char) {
        let mut inner = self.inner.borrow_mut();

        inner.cursor = inner.cursor.min(inner.buffer.len());
        while !inner.buffer.is_char_boundary(inner.cursor) {
            inner.cursor = inner.cursor.saturating_sub(1);
        }

        if inner.buffer.is_empty() {
            inner.buffer.push(ch)
        } else {
            let cursor = inner.cursor;
            inner.buffer.insert(cursor, ch)
        }

        inner.cursor += ch.len_utf8()
    }

    fn select(&mut self, _delta: i32) {}
    fn select_word(&mut self, _delta: i32) {}
    fn delete_word(&mut self, _delta: i32) {}

    fn select_home(&mut self) {
        // how should we do anchors?
    }
    fn select_end(&mut self) {}

    fn move_by(&mut self, delta: i32) {
        let mut inner = self.inner.borrow_mut();
        let mut cursor = inner.cursor as i32;
        let mut remaining = delta.abs();

        let total = inner.buffer.len();
        while remaining > 0 {
            cursor = cursor.saturating_add(delta.signum()).clamp(0, total as i32);
            inner.cursor = cursor as usize;
            if inner.buffer.is_char_boundary(inner.cursor) {
                remaining -= 1;
            }
        }
    }

    fn delete(&mut self, delta: i32) {
        let mut inner = self.inner.borrow_mut();
        let anchor = inner.cursor as i32;
        let mut end = anchor;
        let mut remaining = delta.abs();
        let mut len = 0;

        let total = inner.buffer.len();
        while remaining > 0 {
            end = end.saturating_add(delta.signum()).clamp(0, total as i32);
            len += 1;
            if inner.buffer.is_char_boundary(inner.cursor) {
                remaining -= 1;
            }
        }

        if delta < 0 {
            inner.cursor = inner.cursor.saturating_sub(len)
        }

        let range = anchor.min(end) as usize..anchor.max(end) as usize;
        inner.buffer.replace_range(range, "")
    }

    fn home(&mut self) {
        self.inner.borrow_mut().cursor = 0;
    }

    fn end(&mut self) {
        let mut inner = self.inner.borrow_mut();
        inner.cursor = inner.buffer.len();
    }
}

#[derive(Debug, Default)]
struct Inner {
    buffer: String,
    cursor: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct InputResponse {
    pub consumed: bool,
}

#[derive(Debug, Default)]
pub struct InputWidget {
    props: InputBuffer,
    consume: bool,
}

impl Widget for InputWidget {
    type Response = InputResponse;
    type Props<'a> = (&'a InputBuffer, &'a mut String);

    fn update(&mut self, (props, entered): Self::Props<'_>) -> Self::Response {
        let consumed = std::mem::take(&mut self.consume);
        if consumed {
            *entered = self.props.take();
        }

        self.props = props.clone();
        Self::Response { consumed }
    }

    fn layout(&self, _ctx: LayoutCtx, input: Constraints) -> Size {
        // TODO calculate height
        input.constrain_min(size(20.0, 1.0))
    }

    fn interest(&self) -> Interest {
        Interest::KEY_INPUT | Interest::MOUSE
    }

    fn paint(&self, mut ctx: PaintCtx) {
        ctx.draw(Filled::bg(0x330033));

        let inner = self.props.inner.borrow();
        ctx.draw(Styled::new(&inner.buffer));

        let cursor_pos = pos2(inner.cursor as _, 0) + ctx.rect.left_top();
        if let Some(cell) = ctx.canvas.get_mut(cursor_pos) {
            *cell = cell.attr(Attribute::UNDERLINE).fg(0xFF0000)
        }
    }

    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled {
        let key = match event {
            Event::MouseClick(ev) => {
                let mut inner = self.props.inner.borrow_mut();
                let pos = remap(
                    ev.pos.x as f32,
                    (
                        ctx.rect.left() as f32,
                        ctx.rect.left() as f32 + inner.buffer.len() as f32,
                    ),
                    (0.0, inner.buffer.len() as f32),
                );
                inner.cursor = pos as usize;
                return Handled::Sink;
            }
            Event::MouseDrag(_) => {
                // TODO this
                return Handled::Bubble;
            }
            Event::KeyInput(key) => key,
            _ => return Handled::Bubble,
        };

        // if text is too long
        // if rect height == 1 translate
        // else wrap (this'll involve a re-layout or something) (do we actually add wrapping to the text)
        // (or do we have an adjacency lists of indices we have to keep up to date?)

        // TODO modifiers

        // if event.modifiers().is_some() {
        //     return Handled::Bubble;
        // }

        match key.key {
            Key::Char(ch) if !ch.is_control() => self.props.insert(ch),

            Key::Left if key.modifiers.is_shift() && key.modifiers.is_ctrl() => {
                self.props.select_word(-1)
            }
            Key::Left if key.modifiers.is_shift() => self.props.select(-1),

            Key::Left => self.props.move_by(-1),

            Key::Right if key.modifiers.is_shift() && key.modifiers.is_ctrl() => {
                self.props.select_word(1)
            }
            Key::Right if key.modifiers.is_shift() => self.props.select(1),

            Key::Right => self.props.move_by(1),

            Key::Home if key.modifiers.is_shift() => self.props.select_home(),
            Key::Home => self.props.home(),

            Key::End if key.modifiers.is_shift() => self.props.select_end(),
            Key::End => self.props.end(),

            Key::Backspace if key.modifiers.is_shift() => self.props.delete_word(-1),
            Key::Backspace => self.props.delete(-1),

            Key::Delete if key.modifiers.is_shift() => self.props.delete_word(1),
            Key::Delete => self.props.delete(1),

            Key::Enter => self.consume = true,

            _ => return Handled::Bubble, // Key::Insert | Key::Tab | Key::BackTab
        }

        Handled::Sink
    }
}

// why is there 2 of these?
pub fn text_input(input: &InputBuffer, entered: &mut String) -> Response<InputResponse> {
    InputWidget::show((input, entered))
}
