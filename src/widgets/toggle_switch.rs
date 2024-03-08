use crate::{
    animation::{self, Id},
    color::Rgba,
    context::{LayoutCtx, PaintCtx},
    geom::{
        math::{lerp, remap},
        pos2, size, Constraints, Size,
    },
    paint::{shape::Filled, Cell},
    NoResponse, Widget, WidgetExt as _,
};

use super::on_click;

#[derive(Debug)]
pub struct ToggleSwitch {
    knob: char,
    track: char,
    active_knob: Rgba,
    track_color: Rgba,
    inactive_knob: Option<Rgba>,
    easing: fn(f32) -> f32,
    animation_time: f32,
    id: Id,
    value: bool,
}

impl ToggleSwitch {
    pub fn knob(mut self, knob: char) -> Self {
        self.knob = knob;
        self
    }

    pub fn active_knob(mut self, active_knob: impl Into<Rgba>) -> Self {
        self.active_knob = active_knob.into();
        self
    }

    pub fn inactive_knob(mut self, inactive_knob: impl Into<Rgba>) -> Self {
        self.inactive_knob = Some(inactive_knob.into());
        self
    }

    pub fn track(mut self, track: char) -> Self {
        self.track = track;
        self
    }

    pub fn track_color(mut self, track_color: impl Into<Rgba>) -> Self {
        self.track_color = track_color.into();
        self
    }

    pub const fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub const fn animation_time(mut self, animation_time: f32) -> Self {
        self.animation_time = animation_time;
        self
    }

    pub fn show(mut self, value: &mut bool) {
        self.id = Id::from_ptr(value);
        self.value = *value;
        let resp = on_click(|| ToggleSwitchWidget::show(self));
        *value ^= resp.clicked;
    }
}

impl Default for ToggleSwitch {
    fn default() -> Self {
        Self {
            knob: '█',
            track: '■',
            active_knob: Rgba::from_u32(0x4169E1),
            track_color: Rgba::from_u32(0x333333),
            inactive_knob: None,
            easing: animation::easing::sine_in_out,
            animation_time: 0.3,
            value: false,
            id: Id::EMPTY,
        }
    }
}

#[derive(Default, Debug)]
struct ToggleSwitchWidget {
    props: ToggleSwitch,
}

impl Widget for ToggleSwitchWidget {
    type Response = NoResponse;
    type Props<'a> = ToggleSwitch;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props
    }

    fn layout(&self, _: LayoutCtx, input: Constraints) -> Size {
        input.constrain_min(size(5.0, 1.0))
    }

    fn paint(&self, mut ctx: PaintCtx) {
        let animation =
            ctx.animate_bool(self.props.id, self.props.value, self.props.animation_time);
        let pos = (self.props.easing)(animation);

        // TODO this is just a lerp
        let x = remap(pos, (0.0, 1.0), (ctx.rect.left(), ctx.rect.right()))
            .clamp(ctx.rect.left(), ctx.rect.right());

        ctx.draw(Filled::new(
            Cell::new(self.props.track).fg(self.props.track_color),
        ));

        let blend = lerp(0.0, 0.4, 1.0 - pos);
        let off = pos < 0.5;

        // TODO fade from 1.0..=0.4
        let fg = if off {
            self.props
                .inactive_knob
                .unwrap_or(self.props.active_knob.darken(blend))
        } else {
            self.props.active_knob
        };

        let cell = Cell::new(self.props.knob).fg(fg);
        ctx.put(pos2(x, ctx.rect.top()), cell);
    }
}

pub fn toggle_switch(value: &mut bool) {
    ToggleSwitch::default().show(value)
}
