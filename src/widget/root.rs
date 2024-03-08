use crate::{
    context::LayoutCtx,
    geom::{Constraints, Size},
};

use super::{NoResponse, Widget};

#[derive(Default, Debug)]
pub(crate) struct RootWidget;

impl Widget for RootWidget {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        ctx.new_layer();
        for &child in ctx.children {
            ctx.compute(child, input);
        }
        input.max
    }
}
