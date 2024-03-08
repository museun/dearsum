#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Color {
    Rgba(Rgba),
    Reset,
    #[default]
    Reuse,
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::Rgba(Rgba::from_u32(value))
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        Self::Rgba(value)
    }
}

mod rgba;
pub use rgba::Rgba;

mod hsl;
pub use hsl::Hsl;

mod gradient;
pub use gradient::{gradient, Gradient};
