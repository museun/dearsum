use crate::{
    context::EventCtx,
    geom::{Pos2, Vec2},
    input::{Event, Handled, Interest},
    widget::Response,
    Widget, WidgetExt as _,
};

mod filter;
pub use filter::MouseEventFilter;

#[derive(Copy, Clone, Debug)]
pub struct MouseAreaResponse {
    pub clicked: bool,
    pub hovered: bool,
    pub scrolled: Option<i32>,
    pub dragged: Option<Dragged>,
}

#[derive(Copy, Clone, Debug)]
pub struct Dragged {
    pub current: Pos2,
    pub delta: Vec2,
}

#[derive(Debug, Default)]
enum MouseState {
    #[default]
    None,
    // TODO held
    Hovering,
}

#[derive(Debug, Default)]
struct MouseAreaWidget {
    props: MouseEventFilter,
    state: MouseState,
    clicked: bool,
    scrolled: Option<i32>,
    dragged: Option<Dragged>,
}

impl MouseAreaWidget {
    fn reset(&mut self) {
        std::mem::take(&mut self.state);
        std::mem::take(&mut self.clicked);
        std::mem::take(&mut self.scrolled);
        std::mem::take(&mut self.dragged);
    }
}

impl Widget for MouseAreaWidget {
    type Response = MouseAreaResponse;
    type Props<'a> = MouseEventFilter;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;

        let resp = Self::Response {
            clicked: std::mem::take(&mut self.clicked),
            hovered: matches!(self.state, MouseState::Hovering),
            scrolled: self.scrolled,
            dragged: self.dragged,
        };
        self.reset();
        resp
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE
    }

    fn event(&mut self, _ctx: EventCtx, event: Event) -> Handled {
        self.reset();

        match event {
            Event::MouseEnter(_) if self.props.is_enter() => {
                self.state = MouseState::Hovering;
            }

            Event::MouseLeave(_) if self.props.is_leave() => {}

            // TODO support different buttons for different states
            Event::MouseClick(event) if self.props.is_click() && event.button.is_primary() => {
                self.clicked = true;
                self.state = MouseState::Hovering
            }

            // TODO support different buttons for different states
            Event::MouseHeld(event) if self.props.is_held() && event.button.is_primary() => {
                self.state = MouseState::Hovering
            }

            // TODO support different buttons for different states
            Event::MouseDrag(event) if self.props.is_drag() && event.button.is_primary() => {
                self.dragged = Some(Dragged {
                    current: event.pos,
                    delta: event.delta,
                });
                self.state = MouseState::Hovering
            }

            Event::MouseScroll(event) if self.props.is_scroll() => {
                self.scrolled = Some(event.delta.y)
            }

            _ => {}
        }

        Handled::Bubble
    }
}

pub fn mouse_area<R>(
    filter: MouseEventFilter,
    show: impl FnOnce() -> R,
) -> Response<MouseAreaResponse, R> {
    MouseAreaWidget::show_children(filter, show)
}

pub fn on_click<R>(show: impl FnOnce() -> R) -> Response<MouseAreaResponse, R> {
    mouse_area(MouseEventFilter::empty().click(), show)
}
