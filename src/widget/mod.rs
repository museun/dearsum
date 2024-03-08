use crate::context::{EventCtx, LayoutCtx, PaintCtx};
use crate::geom::{Constraints, FlexFit, Flow, Size};
use crate::input::{Event, Handled, Interest};

mod erased;
pub(crate) use erased::ErasedWidget;

mod widget_ext;
pub use widget_ext::WidgetExt;

mod placeholder;
pub(crate) use placeholder::PlaceholderWidget;

mod root;
pub(crate) use root::RootWidget;

mod response;
pub use response::Response;

pub type NoResponse = ();

pub trait Props {}
impl<T> Props for T {}

// #[cfg(feature = "serde")]
// pub trait Serialize: ::serde::Serialize {}
// #[cfg(feature = "serde")]
// impl<T> Serialize for T where T: ::serde::Serialize {}

// #[cfg(not(feature = "serde"))]
pub trait Serialize {}
// #[cfg(not(feature = "serde"))]
impl<T> Serialize for T {}

pub trait Widget: Default + std::fmt::Debug + 'static + Serialize {
    type Response;
    type Props<'a>: Props;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response;

    // TODO relative_size (default to 100% 100%)

    fn flex(&self) -> (u16, FlexFit) {
        (0, FlexFit::Loose)
    }

    fn flow(&self) -> Flow {
        Flow::Inline
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Size {
        self.default_layout(ctx, input)
    }

    fn paint(&self, ctx: PaintCtx) {
        self.default_paint(ctx);
    }

    fn interest(&self) -> Interest {
        Interest::NONE
    }

    fn event(&mut self, ctx: EventCtx, event: Event) -> Handled {
        let _ = ctx;
        let _ = event;
        Handled::Bubble
    }

    fn default_layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let mut size = Size::ZERO;
        for &child in ctx.children {
            size = size.max(ctx.compute(child, input))
        }
        input.constrain_min(size)
    }

    fn default_paint(&self, mut ctx: PaintCtx) {
        for &child in ctx.children {
            ctx.paint(child)
        }
    }
}
