use crate::{
    color::Rgba,
    context::{EventCtx, PaintCtx},
    geom::Margin,
    input::{Event, Handled, Interest},
    paint::{shape, Label, Styled},
    widget::Response,
    Widget, WidgetExt as _,
};

use super::{label, margin};

#[derive(Copy, Clone, Debug)]
pub struct ButtonResponse {
    pub clicked: bool,
}

#[derive(Debug, Default)]
pub struct Button<T: Label> {
    label: Styled<T>,
    bg: Rgba,
    margin: Margin,
    disabled: bool, // BUG: weird language here
}

impl<T: Label> Button<T> {
    pub fn new(label: impl Into<Styled<T>>) -> Button<T> {
        Button {
            label: label.into(),
            bg: Rgba::from_static("#4C0082"),
            margin: Margin::symmetric(2, 0),
            disabled: false,
        }
    }

    pub fn margin(mut self, margin: impl Into<Margin>) -> Self {
        self.margin = margin.into();
        self
    }

    pub fn bg(mut self, bg: impl Into<Rgba>) -> Self {
        self.bg = bg.into();
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show(self) -> Response<ButtonResponse> {
        ButtonWidget::show_children((self.bg, self.disabled), || {
            margin(self.margin, || {
                label(self.label);
            });
        })
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum ButtonState {
    #[default]
    None,
    Hovered,
    Held,
}

#[derive(Default, Debug)]
struct ButtonWidget {
    props: Rgba,
    state: ButtonState,
    clicked: bool,
    disabled: bool,
}

impl Widget for ButtonWidget {
    type Response = ButtonResponse;
    type Props<'a> = (Rgba, bool);

    fn update(&mut self, (props, disabled): Self::Props<'_>) -> Self::Response {
        self.props = props;
        self.disabled = disabled;

        Self::Response {
            clicked: std::mem::take(&mut self.clicked),
        }
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE & !Interest::MOUSE_MOVE
    }

    fn event(&mut self, _ctx: EventCtx, event: Event) -> Handled {
        std::mem::take(&mut self.state);

        if self.disabled {
            return Handled::Bubble;
        }

        match event {
            Event::MouseEnter(_) => {
                self.state = ButtonState::Hovered;
            }
            Event::MouseLeave(_) => {}
            Event::MouseClick(_) => {
                self.state = ButtonState::Hovered;
                self.clicked = true;
            }
            Event::MouseHeld(_) => {
                self.state = ButtonState::Held;
            }
            _ => return Handled::Bubble,
        }

        Handled::Sink
    }

    fn paint(&self, mut ctx: PaintCtx) {
        let mut bg = match self.state {
            ButtonState::Hovered => self.props.lighten(0.3),
            ButtonState::Held => self.props.darken(0.3),
            _ => self.props,
        };

        if self.disabled {
            bg = Rgba::from_u32(0x333333);
        }

        ctx.draw(shape::Filled::bg(bg));
        self.default_paint(ctx)
    }
}

pub fn button<T: Label>(label: impl Into<Styled<T>>) -> Response<ButtonResponse> {
    Button::new(label).show()
}

pub fn disabled_button<T: Label>(
    disabled: bool,
    label: impl Into<Styled<T>>,
) -> Response<ButtonResponse> {
    Button::new(label).disabled(disabled).show()
}

pub fn color_button<T: Label>(
    label: impl Into<Styled<T>>,
    bg: impl Into<Rgba>,
) -> Response<ButtonResponse> {
    Button::new(label).bg(bg).show()
}
