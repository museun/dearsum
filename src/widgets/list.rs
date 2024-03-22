use crate::{
    context::LayoutCtx,
    geom::{
        size, Constraints, CrossAxisAlignment, FlexFit, Flow, MainAxisAlignment, MainAxisSize, Size,
    },
    widget::UserResponse,
    NoResponse, Widget, WidgetExt as _,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

impl Direction {
    pub const fn size(&self, main: f32, cross: f32) -> Size {
        match self {
            Self::Horizontal => size(main, cross),
            Self::Vertical => size(cross, main),
        }
    }

    pub const fn get_main_axis(&self, size: Size) -> f32 {
        match self {
            Self::Horizontal => size.x,
            Self::Vertical => size.y,
        }
    }

    pub const fn get_cross_axis(&self, size: Size) -> f32 {
        match self {
            Self::Horizontal => size.y,
            Self::Vertical => size.x,
        }
    }
}

#[derive(Debug)]
pub struct List {
    direction: Direction,
    spacing: i32,
    main_axis_size: MainAxisSize,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
}

impl Default for List {
    fn default() -> Self {
        Self::row()
    }
}

impl List {
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            spacing: 0,
            main_axis_size: MainAxisSize::Max,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
        }
    }

    pub const fn row() -> Self {
        Self::new(Direction::Horizontal)
    }

    pub const fn column() -> Self {
        Self::new(Direction::Vertical)
    }

    pub const fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    pub const fn main_axis_size(mut self, main_axis_size: MainAxisSize) -> Self {
        self.main_axis_size = main_axis_size;
        self
    }

    pub const fn main_axis_alignment(mut self, main_axis_alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = main_axis_alignment;
        self
    }

    pub const fn cross_axis_alignment(mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;
        self
    }

    pub fn show<R>(self, show: impl FnOnce() -> R) -> UserResponse<R> {
        ListWidget::show_children(self, show)
    }
}

#[derive(Default, Debug)]
pub struct ListWidget {
    props: List,
}

impl Widget for ListWidget {
    type Response = NoResponse;
    type Props<'a> = List;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props.cross_axis_alignment.flex(), FlexFit::Tight)
    }

    fn layout(&self, mut ctx: LayoutCtx, input: Constraints) -> Size {
        let child_len = ctx.children.len().saturating_sub(1);

        let mut total_main_axis_size = self.props.spacing as f32 * child_len as f32;
        let mut max_cross_axis_size = 0.0_f32;

        let direction = self.props.direction;
        let cross_axis_max = direction.get_cross_axis(input.max);
        let cross_axis_min = match self.props.cross_axis_alignment {
            CrossAxisAlignment::Stretch => cross_axis_max,
            _ => 0.0,
        };

        let mut main_axis_max = direction.get_main_axis(input.max);
        if main_axis_max.is_infinite() {
            main_axis_max = direction.get_main_axis(input.min)
        }

        let mut total_flex = 0.0_f32;
        for &child_id in ctx.children {
            let child = ctx.get_node(child_id);
            let (flex, _) = child.flex();
            total_flex += flex as f32;

            if flex != 0 {
                continue;
            }

            if child.flow().is_relative() {
                continue;
            }

            let constraints = Constraints::new(
                direction.size(0.0, cross_axis_min),
                direction.size(f32::INFINITY, cross_axis_max),
            );
            let size = ctx.compute(child_id, constraints);
            total_main_axis_size += direction.get_main_axis(size);
            max_cross_axis_size = max_cross_axis_size.max(direction.get_cross_axis(size));
        }

        let remaining_main_axis = (main_axis_max - total_main_axis_size).max(0.0);
        for &child_id in ctx.children {
            let child = ctx.get_node(child_id);
            let (flex, fit) = child.flex();
            if flex == 0 {
                continue;
            }

            if child.flow().is_relative() {
                continue;
            }

            let main_axis_size = flex as f32 * remaining_main_axis / total_flex;
            let (min, max) = match fit {
                FlexFit::Loose => (
                    direction.size(0.0, cross_axis_max),
                    direction.size(main_axis_size, cross_axis_max),
                ),
                FlexFit::Tight => (
                    direction.size(main_axis_size, cross_axis_min),
                    direction.size(main_axis_size, cross_axis_max),
                ),
            };
            let constraints = Constraints::new(min, max);
            let size = ctx.compute(child_id, constraints);
            total_main_axis_size += direction.get_main_axis(size);
            max_cross_axis_size = max_cross_axis_size.max(direction.get_cross_axis(size));
        }

        let cross_size = max_cross_axis_size.max(direction.get_cross_axis(input.min));
        let main_axis_size = match self.props.main_axis_size {
            MainAxisSize::Max => total_main_axis_size,
            MainAxisSize::Min => {
                let max = direction.get_main_axis(input.max);
                if max.is_infinite() {
                    total_main_axis_size
                } else {
                    total_main_axis_size.max(max)
                }
            }
        };

        let container = input.constrain(direction.size(main_axis_size, cross_size));

        for &child_id in ctx.children {
            let child = ctx.get_node(child_id);
            let Flow::Relative { anchor, offset } = child.flow() else {
                continue;
            };

            ctx.compute(child_id, Constraints::none());
            let size = (container * anchor) + offset.resolve(container);
            ctx.set_pos(child_id, size.to_pos2());
        }

        let Size {
            x: leading,
            y: mut between,
        } = spacing(
            self.props.main_axis_alignment,
            ctx.children.len(),
            main_axis_size,
            total_main_axis_size,
        );

        between += self.props.spacing as f32;
        let mut next = leading;

        for &child_id in ctx.children {
            if ctx.get_node(child_id).flow().is_relative() {
                continue;
            }

            let layout = ctx.get_layout(child_id);
            let size = layout.rect.size().into();
            let main = direction.get_main_axis(size);
            let cross = direction.get_cross_axis(size);

            use CrossAxisAlignment as C;
            let cross = match self.props.cross_axis_alignment {
                C::Start | C::Stretch => 0.0,
                C::Center => (cross_size - cross) * 0.5,
                C::End => cross_size - cross,
            };

            let size = direction.size(next, cross);
            ctx.set_pos(child_id, size.to_pos2());

            next += main + between
        }

        container
    }
}

fn spacing(alignment: MainAxisAlignment, children: usize, main_size: f32, total_size: f32) -> Size {
    use MainAxisAlignment as M;

    let mut size = Size::ZERO;
    match alignment {
        M::Start => {}
        M::SpaceAround if children == 0 => {}
        M::SpaceBetween if children <= 1 => {}

        M::Center => size.x = (total_size - main_size) * 0.5,
        M::End => size.x = total_size - main_size,

        M::SpaceAround => {
            size.y = (total_size - main_size) / children as f32;
            size.x = size.y * 0.5;
        }

        M::SpaceBetween => {
            size.y = (total_size - main_size) / (children - 1) as f32;
        }

        M::SpaceEvenly => {
            let space = (total_size - main_size) / (children + 1) as f32;
            size.x = space;
            size.y = space;
        }
    };
    size
}

pub fn row<R>(show: impl FnOnce() -> R) -> UserResponse<R> {
    List::row().show(show)
}

pub fn column<R>(show: impl FnOnce() -> R) -> UserResponse<R> {
    List::column().show(show)
}
