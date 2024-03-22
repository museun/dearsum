use std::ops::RangeInclusive;

use crate::{
    color::{Color, Rgba},
    context::{EventCtx, LayoutCtx, PaintCtx},
    geom::{
        math::{almost_eq, remap},
        pos2, size, vec2, Constraints, Rect, Size,
    },
    input::{Event, Handled, Interest},
    paint::{shape::Filled, Cell},
    widget::Response,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Copy, Clone, Debug)]
pub struct SliderStyle {
    pub track: char,
    pub knob: char,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl SliderStyle {
    pub const SOLID: Self = Self {
        // track: '▮',
        track: '■',
        knob: '█',
    };

    pub const FILLED: Self = Self {
        track: '█',
        knob: '█',
    };

    pub const DEFAULT: Self = Self {
        track: '─',
        knob: '●',
    };

    pub const SMALL: Self = Self {
        track: '─',
        knob: '▮',
        // knob: '◆',
    };

    pub const fn track(mut self, char: char) -> Self {
        self.track = char;
        self
    }

    pub const fn knob(mut self, char: char) -> Self {
        self.knob = char;
        self
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Slider {
    min: f32,
    max: f32,

    step_by: Option<f32>,

    track: Color,
    filled: Color,
    knob: Color,

    style: SliderStyle,
}

impl Slider {
    pub const fn new(range: RangeInclusive<f32>) -> Self {
        Self {
            min: *range.start(),
            max: *range.end(),
            step_by: None,

            // TODO redo these with a color pair
            // TODO make these names more apparent what they control
            track: Color::Rgba(Rgba::from_u32(0x555555)),
            filled: Color::Rgba(Rgba::from_u32(0x333333)),
            knob: Color::Rgba(Rgba::from_u32(0xAAAAAA)),

            style: SliderStyle::DEFAULT,
        }
    }

    pub const fn style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    pub fn track(mut self, track: impl Into<Color>) -> Self {
        self.track = track.into();
        self
    }

    pub fn filled(mut self, filled: impl Into<Color>) -> Self {
        self.filled = filled.into();
        self
    }

    pub fn knob(mut self, knob: impl Into<Color>) -> Self {
        self.knob = knob.into();
        self
    }

    pub const fn step_by(mut self, step: f32) -> Self {
        self.step_by = Some(step);
        self
    }

    pub fn show(self, value: &mut f32) -> Response {
        SliderWidget::show((self, value))
    }
}

#[derive(Debug, Default)]
struct SliderWidget {
    props: Slider,
    value: Option<f32>,
    // for widgets that change their bindings we compare the *mut f32 to this
    ptr: usize,
}

impl Widget for SliderWidget {
    type Response = NoResponse;
    type Props<'a> = (Slider, &'a mut f32);

    fn update(&mut self, (props, value): Self::Props<'_>) -> Self::Response {
        self.props = props;

        let id = value as *const f32 as usize;
        if self.ptr != id {
            self.value = Some(*value);
            self.ptr = id;
            return;
        }

        let cached = *self.value.get_or_insert(*value);
        if !almost_eq(*value, cached) {
            *value = cached
        }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE
    }

    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled {
        let Event::MouseDrag(event) = event else {
            return Handled::Bubble;
        };

        // TODO not like this
        let mut value = remap(
            event.pos.x as f32,
            (ctx.rect.left() as f32, ctx.rect.right() as f32),
            (self.props.min, self.props.max),
        )
        .clamp(self.props.min, self.props.max);

        if let Some(step) = self.props.step_by {
            value = round_to_step(value, step)
        }
        self.value.replace(value);

        Handled::Sink
    }

    fn layout(&self, _: LayoutCtx, input: Constraints) -> Size {
        input.constrain_min(size(20.0, 1.0))
    }

    fn paint(&self, mut ctx: PaintCtx) {
        let props = self.props;

        // TODO these names don't match up
        let track_cell = Cell::new(props.style.track).fg(props.filled);
        let remaining_cell = Cell::new(props.style.track).fg(props.track);
        let knob_cell = Cell::new(props.style.knob).fg(props.knob);

        ctx.draw(Filled::new(track_cell));

        let min = ctx.rect.left();
        let max = ctx.rect.right();

        // FIXME this needs to implement rounding and smoothing
        let x = remap(
            self.value.unwrap_or_default(),
            (self.props.min, self.props.max),
            (min, max),
        );

        ctx.draw_cropped(
            Rect::from_min_size(ctx.rect.min, vec2(x.saturating_sub_unsigned(1), 1)),
            Filled::new(remaining_cell),
        );

        ctx.put(pos2(x, ctx.rect.top()), knob_cell)
    }
}

fn round_to_step(value: f32, step: f32) -> f32 {
    if step == 0.0 {
        return value;
    }
    (value / step).round() * step
}

pub fn slider(current: &mut f32, range: RangeInclusive<f32>) -> Response {
    Slider::new(range).show(current)
}
