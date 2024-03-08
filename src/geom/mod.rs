mod pos2;
pub use pos2::{pos2, Pos2};

mod vec2;
pub use vec2::{vec2, Vec2};

mod rect;
pub use rect::{rect, Rect};

mod align;
pub use align::{Align, Align2};

mod margin;
pub use margin::Margin;

mod vec3;
pub use vec3::{vec3, Vec3};

pub mod math;

mod constraints;
pub use constraints::{
    size, Constraints, CrossAxisAlignment, Dimension, Dimension2, FlexFit, Flow, MainAxisAlignment,
    MainAxisSize, Size,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

impl std::ops::Not for Axis {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.swap();
        self
    }
}

impl Axis {
    pub fn swap(&mut self) {
        *self = match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}
