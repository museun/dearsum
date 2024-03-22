use crate::{
    debug_fmt::short_name,
    geom::{vec2, Align, Rect},
    node::{LayoutNode, Node, WidgetId},
    paint::{DebugRenderer, Surface},
    ui::Ui,
};

use slotmap::{SecondaryMap, SlotMap};

#[derive(Default, Clone)]
pub struct DebugNode {
    pub id: WidgetId,
    pub name: String,
    pub debug: String,
    pub rect: Rect,
    pub children: Vec<Self>,
}

impl DebugNode {
    pub fn build(ui: &Ui) -> Self {
        fn build(
            debug_nodes: &mut Vec<DebugNode>,
            id: WidgetId,
            nodes: &SlotMap<WidgetId, Node>,
            layout: &SecondaryMap<WidgetId, LayoutNode>,
        ) {
            let mut children = vec![];

            for &child in &nodes[id].children {
                build(&mut children, child, nodes, layout)
            }

            debug_nodes.push(DebugNode {
                id,
                name: short_name(nodes[id].widget.type_name()),
                rect: layout[id].rect,
                children,
                debug: format!("{:#?}", nodes[id].widget),
            });
        }

        let nodes = ui.nodes();
        let layout = ui.computed();

        let mut children = vec![];
        let root = ui.root();

        build(&mut children, nodes[root].children[0], &nodes, &layout);

        Self {
            id: root,
            name: short_name(nodes[root].widget.type_name()),
            rect: layout[root].rect,
            children,
            debug: format!("{:#?}", nodes[root].widget),
        }
    }

    pub fn flow_tree(&self) -> String {
        #[derive(Copy, Clone, PartialEq, Debug)]
        struct Pos {
            x: usize,
            y: usize,
        }

        impl Pos {
            const ZERO: Self = Self { x: 0, y: 0 };
        }

        #[derive(Debug)]
        enum TreeLabel {
            Separator,
            Label(Label),
        }

        #[derive(Debug)]
        struct Label {
            align: Align,
            label: String,
        }

        impl TreeLabel {
            pub fn new(s: impl ToString, align: Align) -> Self {
                Self::Label(Label {
                    align,
                    label: s.to_string(),
                })
            }

            pub fn split(&self) -> impl Iterator<Item = Self> + '_ {
                let mut a = None;
                let mut b = None;
                match self {
                    Self::Separator => a = Some(std::iter::once(Self::Separator)),
                    TreeLabel::Label(s) => {
                        b = Some(s.label.split('\n').map(|x| Self::new(x, s.align)))
                    }
                }

                std::iter::from_fn(move || match self {
                    TreeLabel::Separator => a.as_mut()?.next(),
                    TreeLabel::Label(_) => b.as_mut()?.next(),
                })
            }

            pub fn len(&self) -> usize {
                match self {
                    Self::Separator => 0, // flex
                    Self::Label(l) => l.label.len(),
                }
            }
        }

        #[derive(Debug)]
        struct TreeNode {
            center: usize,
            width: usize,
            height: usize,
            total_width: usize,
            total_height: usize,
            labels: Vec<TreeLabel>,
            children: Vec<Self>,
        }

        impl TreeNode {
            pub fn new(node: &DebugNode, spacing: usize) -> Self {
                let id = format!("{:?}", crate::debug_fmt::id(node.id));
                let rect = format!(
                    " x: {} y: {}, w: {} h: {} ",
                    node.rect.left(),
                    node.rect.top(),
                    node.rect.width(),
                    node.rect.height()
                );

                let labels = [
                    TreeLabel::new(id, Align::Center),
                    TreeLabel::Separator,
                    TreeLabel::new(rect, Align::Center),
                    TreeLabel::Separator,
                    TreeLabel::new(&node.debug, Align::Min),
                ];

                let labels = labels.iter().flat_map(|s| s.split()).collect::<Vec<_>>();

                let node_width = labels.iter().map(|x| x.len()).max().unwrap() + 4;
                let node_height = labels.len() + 2;

                let children = node
                    .children
                    .iter()
                    .map(|x| Self::new(x, spacing))
                    .collect::<Vec<_>>();

                let children_width = Self::compute_children_width(&children, spacing);

                let total_width = std::cmp::max(node_width, children_width);

                let mut total_height = node_height;
                if !node.children.is_empty() {
                    let children_height =
                        children.iter().map(|c| c.total_height).max().unwrap_or(0);

                    total_height = if node.children.len() == 1 {
                        node_height + children_height
                    } else {
                        node_height + children_height + 1
                    }
                }

                let cx = (node_width - 1) / 2;

                let center = match (children.first(), children.last()) {
                    (Some(first), Some(last)) => {
                        cx.max(first.center + children_width - last.total_width + last.center) / 2
                    }
                    _ => cx,
                };

                Self {
                    center,
                    width: node_width,
                    height: node_height,
                    total_width,
                    total_height,
                    labels,
                    children,
                }
            }

            fn compute_children_width(children: &[Self], spacing: usize) -> usize {
                if children.is_empty() {
                    return 0;
                }
                children.iter().map(|c| c.total_width).sum::<usize>()
                    + (children.len() - 1) * spacing
            }

            fn inner(&self, buffer: &mut Vec<Vec<char>>, origin: Pos, spacing: usize) {
                let left = origin.x + self.center.saturating_sub((self.width - 1) / 2);
                let right = left + self.width;

                for x in left + 1..right - 1 {
                    buffer[origin.y][x] = '─';
                    buffer[origin.y + self.height - 1][x] = '─';
                }
                #[allow(clippy::needless_range_loop)]
                for y in origin.y + 1..origin.y + self.height - 1 {
                    buffer[y][left] = '│';
                    buffer[y][right - 1] = '│';
                }

                // ╭
                // ╮
                // ╯
                // ╰

                // ┌
                // ┐
                // └
                // ┘

                buffer[origin.y][left] = '╭';
                buffer[origin.y][right - 1] = '╮';
                buffer[origin.y + self.height - 1][left] = '╰';
                buffer[origin.y + self.height - 1][right - 1] = '╯';

                for (row, label) in self.labels.iter().enumerate() {
                    match label {
                        TreeLabel::Separator => {
                            buffer[origin.y + row + 1][left] = '┝';
                            buffer[origin.y + row + 1][left + self.width - 1] = '┥';

                            for i in 1..self.width - 1 {
                                buffer[origin.y + row + 1][left + i] = '┈';
                            }
                        }
                        TreeLabel::Label(label) => {
                            let x = match label.align {
                                Align::Min => 2,
                                Align::Center => (self.width - label.label.len()) / 2,
                                // TODO this
                                Align::Max => 2,
                            };

                            let start = left + x;
                            for (i, ch) in label.label.chars().enumerate() {
                                buffer[origin.y + row + 1][start + i] = ch;
                            }
                        }
                    }
                }

                if origin != Pos::ZERO {
                    buffer[origin.y][origin.x + self.center] = '┷' // end
                }

                self.render_children(buffer, origin, spacing)
            }

            fn render_children(&self, buffer: &mut Vec<Vec<char>>, origin: Pos, spacing: usize) {
                if self.children.is_empty() {
                    return;
                }

                buffer[origin.y + self.height - 1][origin.x + self.center] = '┯'; // start

                let child_origin_y = if self.children.len() > 1 {
                    origin.y + self.height + 1
                } else {
                    origin.y + self.height
                };

                let children = Self::compute_children_width(&self.children, spacing);

                let child_origin_x = if self.children.is_empty() || children > self.width {
                    origin.x
                } else {
                    origin.x + (self.width - children) / 2
                };

                let mut child_origin = Pos {
                    x: child_origin_x,
                    y: child_origin_y,
                };

                for id in 0..self.children.len() {
                    let child = &self.children[id];
                    child.inner(buffer, child_origin, spacing);

                    if id == self.children.len() - 1 {
                        continue;
                    }

                    let start = child_origin.x + child.center + 1;
                    let end =
                        child_origin.x + child.total_width + spacing + self.children[id + 1].center;

                    for x in start..end {
                        let arrow = if x != origin.x + self.center {
                            '╌' // '─'
                        } else {
                            '┴'
                        };
                        buffer[origin.y + self.height][x] = arrow
                    }

                    if id == 0 {
                        buffer[origin.y + self.height][start - 1] = '┌'
                    }

                    buffer[origin.y + self.height][end] = if id == self.children.len() - 2 {
                        '┐'
                    } else if end == origin.x + self.center {
                        '┼'
                    } else {
                        '┬'
                    };

                    child_origin.x += child.total_width + spacing
                }
            }

            pub fn render(&self, spacing: usize) -> String {
                let mut canvas = vec![vec![' '; self.total_width]; self.total_height];

                self.inner(&mut canvas, Pos::ZERO, spacing);

                canvas.into_iter().fold(String::new(), |mut a, mut c| {
                    if !a.is_empty() {
                        a.push('\n');
                    }
                    let p = c.iter().rposition(|&c| c != ' ').unwrap_or(0);
                    c.truncate(p + 1);
                    a.extend(c);
                    a
                })
            }
        }

        TreeNode::new(self, 1).render(1)
    }

    // TODO this is broken
    pub fn ascii_tree(&self) -> String {
        use std::fmt::Write as _;

        fn print(children: &[DebugNode], prefix: &str, out: &mut impl std::fmt::Write) {
            for (i, node) in children.iter().enumerate() {
                if i < children.len() - 1 {
                    let _ = writeln!(out, "{prefix}├─┬╼ {}", node.name);
                    print(&node.children, &format!("{prefix}│ "), out)
                } else if i < children.len()
                    && children.last().filter(|c| c.children.is_empty()).is_some()
                {
                    let _ = writeln!(out, "{prefix}╰╼ {}", node.name);
                    print(&node.children, &format!("{prefix}  "), out)
                } else {
                    let _ = writeln!(out, "{prefix}╰─┬╼ {}", node.name);
                    print(&node.children, &format!("{prefix}  "), out)
                }
            }
        }

        let mut out = String::new();
        let _ = writeln!(out, "{}", self.name);
        print(&self.children, "", &mut out);

        out
    }
}

impl std::fmt::Debug for DebugNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DebugNode")
            .field("id", &crate::debug_fmt::id(self.id))
            .field("name", &crate::debug_fmt::str(&self.name))
            .field("rect", &crate::debug_fmt::rect(self.rect))
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug)]
pub struct DebugOutput {
    pub nodes: String,
    pub layout: String,
    pub render: String,
    pub node: DebugNode,
}

impl std::fmt::Display for DebugOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.nodes)?;
        writeln!(f, "{}", self.layout)?;
        writeln!(f, "{}", self.render)
    }
}

pub fn evaluate<R>(mut app: impl FnMut(&Ui) -> R) -> DebugOutput {
    let mut surface = Surface::new(vec2(80, 25));
    let ui = Ui::new(surface.rect());

    ui.scope(|| app(&ui)).unwrap();
    ui.paint(&mut surface);

    let mut debug = DebugRenderer::default();
    surface.render(&mut debug).unwrap();

    let rect = surface.rect();

    use crate::debug_fmt::{secondary_map, slot_map};
    DebugOutput {
        nodes: format!("{:#?}", slot_map(&ui.nodes())),
        layout: format!("{:#?}", secondary_map(&ui.computed())),
        render: format!(
            "width: {} height: {}\n{}",
            rect.width(),
            rect.height(),
            debug.out
        ),
        node: DebugNode::build(&ui),
    }
}
