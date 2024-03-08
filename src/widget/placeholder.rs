use super::{NoResponse, Widget};

#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize,))]
pub(crate) struct PlaceholderWidget;

impl Widget for PlaceholderWidget {
    type Response = NoResponse;
    type Props<'a> = ();

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {}
}
