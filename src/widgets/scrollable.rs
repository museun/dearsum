use std::cell::Cell as StdCell;

use crate::{
    context::{EventCtx, LayoutCtx, PaintCtx},
    geom::{math::remap, pos2, size, vec2, Constraints, Rect, Size, Vec2},
    input::{Event, Handled, Interest, Key, KeyPressed},
    paint::{shape::Filled, Cell},
    widget::UserResponse,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Default, Debug)]
pub struct Scrollable {
    stick_to_bottom: bool,
    show_scrollbar: bool,
}

impl Scrollable {
    pub const fn new() -> Self {
        Self {
            stick_to_bottom: false,
            show_scrollbar: true,
        }
    }

    pub const fn show_scrollbar(mut self, show_scrollbar: bool) -> Self {
        self.show_scrollbar = show_scrollbar;
        self
    }

    pub const fn stick_to_bottom(mut self, stick_to_bottom: bool) -> Self {
        self.stick_to_bottom = stick_to_bottom;
        self
    }

    pub fn show<R>(self, show: impl FnOnce() -> R) -> UserResponse<R> {
        ScrollableWidget::show_children(self, show)
    }
}

#[derive(Default, Debug)]
pub struct ScrollableWidget {
    // TODO this needs to be redone
    stick_to_bottom: bool,
    show_scrollbar: bool,
    pos: i32, // TODO horizontal scrolling
    canvas_size: StdCell<Vec2>,
    our_rect: Rect,
}

impl ScrollableWidget {
    fn max(&self) -> i32 {
        self.our_rect.height() - self.canvas_size.get().y
    }
}

impl Widget for ScrollableWidget {
    type Response = NoResponse;
    type Props<'a> = Scrollable;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.stick_to_bottom = props.stick_to_bottom;
        self.show_scrollbar = props.show_scrollbar;
        if self.stick_to_bottom {
            self.pos = self.max();
        }
    }

    fn layout(&self, mut ctx: LayoutCtx, mut input: Constraints) -> Size {
        ctx.enable_clipping();

        let margin = if self.pos < 0 && self.show_scrollbar {
            1.0
        } else {
            0.0
        };
        let margin = size(margin, 0.0);
        input.max -= margin;

        let constraints = Constraints::new(
            size(input.min.x, 0.0), //
            size(input.max.x, f32::INFINITY),
        );

        let mut size = Size::ZERO;
        for &child in ctx.children {
            size = size.max(ctx.compute(child, constraints));
        }

        for &child in ctx.children {
            ctx.set_size(child, vec2(0, self.pos));
        }

        self.canvas_size.set(size.to_vec2());
        input.max += margin;
        input.constrain(size)
    }

    // TODO colors and styling
    // TODO finish this
    fn paint(&self, mut ctx: PaintCtx) {
        if self.pos < 0 && self.show_scrollbar {
            let area = ctx.rect;
            let _rect = Rect::from_min_size(area.right_top(), vec2(1, area.height()));

            ctx.draw(Filled::new(Cell::new('│').fg(0x111111)));

            let y = remap(
                self.pos.abs(),
                (0, self.max().abs()),
                (area.top() as f32, area.bottom() as f32),
            );

            ctx.put(pos2(area.right(), y as i32), Cell::new('┃').fg(0xFFFFFF))
        }

        self.default_paint(ctx)
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE | Interest::KEY_INPUT
    }

    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled {
        self.our_rect = ctx.rect;

        // TODO handle scroll bar events
        let delta = match event {
            Event::MouseDrag(drag) if !drag.released => drag.delta.y,
            Event::MouseScroll(scroll) => -scroll.delta.y,
            // TODO if we have focus
            // TODO focus
            Event::KeyInput(KeyPressed { key, .. }) => match key {
                Key::Up => 1,
                Key::Down => -1,
                Key::PageUp => self.our_rect.height(),
                Key::PageDown => -self.our_rect.height(),
                Key::Home => i32::MAX,
                Key::End => i32::MIN,
                _ => return Handled::Bubble,
            },
            _ => return Handled::Bubble,
        };

        self.pos = (self.pos + delta).min(0).max(self.max());

        Handled::Sink
    }
}

pub fn scrollable<R>(show: impl FnOnce() -> R) -> UserResponse<R> {
    Scrollable::new().show(show)
}
