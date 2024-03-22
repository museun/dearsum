use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    color::Rgba,
    context::{EventCtx, LayoutCtx, PaintCtx},
    geom::{pos2, size, vec2, Constraints, Pos2, Rect, Size, Vec2},
    input::{Event, Handled, Interest, Key, KeyPressed, MouseClick, MouseDrag},
    paint::{render, shape::Filled, Attribute},
    widget::Response,
    Widget, WidgetExt as _,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum WordSep {
    Space,
    Punctuation,
    Other,
}

impl WordSep {
    const fn new(c: char) -> Self {
        match c {
            c if c.is_ascii_whitespace() => Self::Space,
            c if c.is_ascii_punctuation() => Self::Punctuation,
            _ => Self::Other,
        }
    }

    fn find_next_word_start(line: &str, start: usize) -> Option<usize> {
        let mut chars = line.chars().enumerate().skip(start);
        let mut previous = chars.next().map(|(_, i)| Self::new(i))?;
        for (i, c) in chars {
            let current = Self::new(c);
            if current != Self::Space && previous != current {
                return Some(i);
            }
            previous = current;
        }
        None
    }

    fn find_next_word_end(line: &str, start: usize) -> Option<usize> {
        let mut chars = line.chars().enumerate().skip(start);
        let mut previous = chars.next().map(|(_, i)| Self::new(i))?;
        for (i, c) in chars {
            let current = Self::new(c);
            if previous != Self::Space && previous != current {
                return Some(i);
            }
            previous = current;
        }
        None
    }

    fn find_prev_word(line: &str, start: usize) -> Option<usize> {
        let p = line
            .char_indices()
            .nth(start)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let mut chars = line[..p].chars().rev().enumerate();
        let mut current = chars.next().map(|(_, i)| Self::new(i))?;
        for (i, c) in chars {
            let next = Self::new(c);
            if current != Self::Space && next != current {
                return Some(start - i);
            }
            current = next;
        }
        (current != Self::Space).then_some(0)
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputBuffer {
    inner: Rc<RefCell<Inner>>,
}

impl InputBuffer {
    fn measure(&self, width: i32) -> Vec2 {
        let mut size = vec2(0, 1);
        let mut max_x = 0;
        for _ in self.inner.borrow().buffer.chars() {
            max_x += 1;
            if max_x >= width {
                size.y += 1;
                size.x = width;
                max_x = 0;
            }
        }
        size.x = size.x.max(width);
        size
    }
}

const fn translate_cursor(cursor: i32, width: i32) -> Pos2 {
    pos2(cursor % width, cursor / width)
}

const fn cursor_as_index(pos: Pos2, width: i32) -> i32 {
    pos.y * width + pos.x
}

impl InputBuffer {
    pub fn new(buffer: impl ToString) -> Self {
        let buffer = buffer.to_string();
        Self {
            inner: Rc::new(RefCell::new(Inner {
                cursor: buffer.len(),
                buffer,
                anchor: None,
            })),
        }
    }

    pub fn append(&self, data: &str) {
        let mut this = self.inner.borrow_mut();
        this.buffer.push_str(data);
        this.cursor += data.len();
    }

    pub fn set(&self, data: impl ToString) {
        let buffer = data.to_string();
        let data = Inner {
            cursor: buffer.len(),
            buffer,
            anchor: None,
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
    fn take(&self) -> String {
        self.reset_anchor();

        let mut inner = self.inner.borrow_mut();
        inner.cursor = 0;
        std::mem::take(&mut inner.buffer)
    }

    fn insert(&self, ch: char) {
        self.reset_anchor();

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

    fn select(&self, delta: i32) {
        let anchor = {
            let this = self.inner.borrow();
            let cursor = this.cursor;
            this.anchor.unwrap_or(cursor as i32)
        };
        self.select_range(anchor + delta)
    }

    fn select_word(&self, _delta: i32) {
        // TODO
    }

    fn select_home(&self) {
        self.select_range(0)
    }

    fn select_end(&self) {
        let len = self.len();
        self.select_range(len as _)
    }

    fn select_range(&self, delta: i32) {
        let mut this = self.inner.borrow_mut();
        let len = this.buffer.len();
        let value = delta.max(0).min(len as _);
        if value == this.cursor as i32 {
            this.anchor.take();
        } else {
            this.anchor.replace(value);
        }
    }

    fn delete_word(&self, delta: i32) {
        self.reset_anchor();

        let mut this = self.inner.borrow_mut();
        let data = &this.buffer;
        let cursor = this.cursor;

        match delta {
            d if d.is_positive() => {
                let p = WordSep::find_next_word_end(data, cursor).unwrap_or(data.len());
                this.buffer.replace_range(cursor..p, "");
            }
            d if d.is_negative() => {
                let p = WordSep::find_prev_word(data, cursor).unwrap_or(0);
                this.buffer.replace_range(p..cursor, "");
                this.cursor = p;
            }
            _ => {}
        };
    }

    fn reset_anchor(&self) {
        self.inner.borrow_mut().anchor.take();
    }

    fn move_word(&self, delta: i32) {
        self.reset_anchor();

        self.inner.borrow_mut().cursor = match delta {
            d if d.is_positive() => {
                let data = self.as_str();
                let cursor = self.inner.borrow().cursor;
                WordSep::find_next_word_start(&data, cursor).unwrap_or(data.len())
            }
            d if d.is_negative() => {
                let cursor = self.inner.borrow().cursor;
                WordSep::find_prev_word(&self.as_str(), cursor).unwrap_or(0)
            }
            _ => return,
        }
    }

    fn select_next_line(&self, delta: i32, width: i32) {
        let size = self.measure(width as _).to_size();
        let cursor = self.inner.borrow().cursor;

        let mut pos = translate_cursor(cursor as _, width);
        pos.y += delta;
        pos.y = pos.y.max(0).min((size.y) as i32);

        let target = cursor_as_index(pos, width as _);
        self.select_range(target)
    }

    fn move_next_line(&self, delta: i32, width: i32) {
        self.reset_anchor();

        let size = self.measure(width as _).to_size();
        let mut inner = self.inner.borrow_mut();

        let mut pos = translate_cursor(inner.cursor as _, width);
        pos.y += delta;
        pos.y = pos.y.max(0).min((size.y - 1.0) as i32);

        inner.cursor = cursor_as_index(pos, width as _) as usize;
    }

    fn escape(&self) {
        let mut this = self.inner.borrow_mut();
        if let Some(p) = this.anchor.take() {
            this.cursor = p as usize;
        }
    }

    fn move_by(&self, delta: i32) {
        self.reset_anchor();

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

    fn delete(&self, delta: i32) {
        self.reset_anchor();

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
        inner.buffer.replace_range(range, "");
    }

    fn home(&self) {
        self.reset_anchor();

        self.inner.borrow_mut().cursor = 0;
    }

    fn end(&self) {
        self.reset_anchor();

        let mut inner = self.inner.borrow_mut();
        inner.cursor = inner.buffer.len();
    }
}

#[derive(Debug, Default)]
struct Inner {
    buffer: String,
    cursor: usize,
    anchor: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct InputResponse {
    pub consumed: Option<String>,
}

#[derive(Debug, Default)]
pub struct InputWidget {
    props: InputBuffer,
    consume: bool,
}

impl InputWidget {
    fn handle_key(&mut self, ctx: EventCtx, key: KeyPressed) -> Handled {
        match key.key {
            Key::Char('w') if key.modifiers.is_ctrl() => self.props.delete_word(-1),

            Key::Char(ch) if !ch.is_control() => self.props.insert(ch),

            Key::Left if key.modifiers.is_shift() && key.modifiers.is_ctrl() => {
                self.props.select_word(-1)
            }
            Key::Left if key.modifiers.is_shift() => self.props.select(-1),

            Key::Left if key.modifiers.is_ctrl() => self.props.move_word(-1),
            Key::Left => self.props.move_by(-1),

            Key::Right if key.modifiers.is_ctrl() => self.props.move_word(1),
            Key::Right if key.modifiers.is_shift() && key.modifiers.is_ctrl() => {
                self.props.select_word(1)
            }
            Key::Right if key.modifiers.is_shift() => self.props.select(1),

            Key::Right => self.props.move_by(1),

            Key::Escape => self.props.escape(),

            Key::Up if key.modifiers.is_shift() => {
                self.props.select_next_line(-1, ctx.rect.width() as _)
            }
            Key::Down if key.modifiers.is_shift() => {
                self.props.select_next_line(1, ctx.rect.width() as _)
            }

            Key::Up => self.props.move_next_line(-1, ctx.rect.width() as _),
            Key::Down => self.props.move_next_line(1, ctx.rect.width() as _),

            Key::Home if key.modifiers.is_shift() => self.props.select_home(),
            Key::Home => self.props.home(),

            Key::End if key.modifiers.is_shift() => self.props.select_end(),
            Key::End => self.props.end(),

            Key::Backspace => self.props.delete(-1),

            Key::Delete if key.modifiers.is_shift() => self.props.delete_word(1),
            Key::Delete => self.props.delete(1),

            Key::Enter => self.consume = true,

            _ => return Handled::Bubble, // Key::Insert | Key::Tab | Key::BackTab
        }

        Handled::Sink
    }

    fn handle_mouse_click(&mut self, ctx: EventCtx, ev: MouseClick) -> Handled {
        self.props.reset_anchor();
        self.select_cursor(ev.pos, &ctx);
        Handled::Sink
    }

    fn select_cursor(&mut self, pos: Pos2, ctx: &EventCtx) {
        let mut inner = self.props.inner.borrow_mut();
        let pos = pos - ctx.rect.left_top();
        let index = cursor_as_index(pos, ctx.rect.width());
        inner.cursor = (index as usize).min(inner.buffer.len());
    }

    fn handle_mouse_drag(&mut self, ctx: EventCtx, ev: MouseDrag) -> Handled {
        if ev.released {
            return Handled::Sink;
        }

        // FIXME: this is off by 1
        self.select_cursor(ev.origin, &ctx);

        let pos = ev.pos - ctx.rect.left_top();
        let p = cursor_as_index(pos, ctx.rect.width());

        self.props.inner.borrow_mut().anchor = Some(p);
        self.props.select_range(p);

        Handled::Sink
    }

    fn draw_single_line(&self, ctx: &mut PaintCtx) {
        let rect = ctx.rect;
        let width = rect.width();

        let size = self.props.measure(width);

        let inner = self.props.inner.borrow();
        let text = inner.buffer.as_str();

        if size.y == 1 {
            ctx.draw(Filled::bg(0x330033));
            render(text, rect, |pos, cell| ctx.put(pos, cell));
            Self::render_cursor_at(&inner, ctx, rect);
            return;
        }

        ctx.draw(Filled::bg(0xBC8F8F));

        let width = (width as usize).saturating_sub(1);
        let cursor = inner.cursor;
        let start = cursor.saturating_sub(width);
        let end = text.len().max(cursor);
        render(&text[start..end], rect, |pos, cell| ctx.put(pos, cell));

        let offset = cursor.max(0).min(width) as i32;
        let pos = pos2(offset, 0) + ctx.rect.left_top();

        if !ctx.rect.contains(pos) {
            return;
        }

        if let Some(cell) = ctx.canvas.get_mut(pos) {
            cell.patch(|cell| {
                cell.attr(Attribute::UNDERLINE).fg(0xFF0000) //
            });
        }

        // Self::render_cursor_at(&inner, ctx, ctx.rect)
    }

    fn draw_multi_line(&self, ctx: &mut PaintCtx) {
        let mut rect = ctx.rect;
        let w = rect.width() as usize;

        let inner = self.props.inner.borrow();
        let mut text = inner.buffer.as_str();

        while w < text.len() {
            if rect.height() == 0 {
                break;
            }

            // TODO split at word (or if word >= w, at the edge)
            let (head, tail) = text.split_at(w);
            render(head, rect, |pos, cell| ctx.put(pos, cell));
            text = tail;
            rect = (rect + pos2(0, 1)) + vec2(0, -1);
        }

        if rect.height() > 0 && !text.trim().is_empty() {
            render(text, rect, |pos, cell| ctx.put(pos, cell));
        }

        Self::render_cursor_at(&inner, ctx, ctx.rect)
    }

    fn render_selection_range(ctx: &mut PaintCtx, width: i32, cursor: i32, anchor: i32) {
        let start = anchor.min(cursor);
        let end = anchor.max(cursor);
        for pos in start..=end {
            let mut anchor_pos = translate_cursor(pos, width);
            anchor_pos += ctx.rect.left_top();
            if let Some(cell) = ctx.canvas.get_mut(anchor_pos) {
                cell.patch(|cell| cell.fg(u32::MIN).bg(Rgba::from_u32(0x6494ED).lighten(0.2)))
            }
        }

        let pos = translate_cursor(anchor, width) + ctx.rect.left_top();
        if !ctx.rect.contains(pos) {
            return;
        }
        if let Some(cell) = ctx.canvas.get_mut(pos) {
            cell.patch(|cell| cell.bg(0x6494ED));
        }
    }

    // TODO what part of the rect is actually needed for this?
    fn render_cursor_at(inner: &Inner, ctx: &mut PaintCtx, rect: Rect) {
        let width = rect.width();
        let cursor = inner.cursor as i32;

        if let Some(anchor) = inner.anchor.filter(|&anchor| anchor != cursor) {
            Self::render_selection_range(ctx, width, cursor, anchor);
            return;
        }

        // XXX should index on a croppedcanvas automatically translate the position?
        let pos = translate_cursor(cursor, width) + ctx.rect.left_top();

        if let Some(cell) = ctx.canvas.get_mut(pos) {
            cell.patch(|cell| cell.attr(Attribute::UNDERLINE).fg(0xFF0000));
        }
    }
}

impl Widget for InputWidget {
    type Response = InputResponse;
    type Props<'a> = &'a InputBuffer;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        let consumed = std::mem::take(&mut self.consume);
        let consumed = consumed
            .then(|| self.props.take())
            .filter(|c| !c.trim().is_empty());
        self.props = props.clone();
        Self::Response { consumed }
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Size {
        let w = if input.max.x.is_finite() {
            input.max.x
        } else {
            ctx.size.x
        };

        let h = if input.max.y.is_finite() {
            input.max.y
        } else {
            ctx.size.y
        };

        let computed = self.props.measure(w as i32).to_size();
        let base = computed.max(size(w, h.max(1.0)));
        input.constrain(base)
    }

    fn interest(&self) -> Interest {
        Interest::KEY_INPUT | Interest::MOUSE
    }

    fn paint(&self, mut ctx: PaintCtx) {
        if ctx.rect.height() == 0 {
            return;
        }

        // TODO style for this

        // TODO should this be an option on the props?
        if ctx.rect.height() == 1 {
            self.draw_single_line(&mut ctx);
            return;
        }

        ctx.draw(Filled::bg(0x473D8B));
        self.draw_multi_line(&mut ctx);
    }

    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled {
        match event {
            Event::MouseClick(ev) => self.handle_mouse_click(ctx, ev),
            Event::MouseDrag(ev) => self.handle_mouse_drag(ctx, ev),
            Event::KeyInput(ev) => self.handle_key(ctx, ev),
            _ => Handled::Bubble,
        }
    }
}

pub fn text_input(input: &InputBuffer) -> Response<InputResponse> {
    InputWidget::show(input)
}
