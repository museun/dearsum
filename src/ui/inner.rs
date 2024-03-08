use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::TypeId,
    cell::{Cell, Ref, RefCell, RefMut},
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::{
    animation,
    context::LayoutCtx,
    geom::{Constraints, Pos2, Rect, Vec2},
    input::{Handled, Input},
    node::{LayoutNode, Node, WidgetId},
    paint::Surface,
    terminal::{event::Event, Terminal},
    ui::{context, paint::Paint, Command, Response, Ui},
    widget::{ErasedWidget, PlaceholderWidget, RootWidget, Widget},
};

use super::Layout;

#[derive(Default)]
pub struct Inner {
    pub nodes: RefCell<SlotMap<WidgetId, Node>>,
    pub computed: RefCell<SecondaryMap<WidgetId, LayoutNode>>,

    pub input: RefCell<Input>,
    pub clip_stack: RefCell<Vec<WidgetId>>,

    pub root: WidgetId,

    pub stack: RefCell<Vec<WidgetId>>,
    pub removed: RefCell<Vec<WidgetId>>,

    pub rect: Cell<Rect>,
    pub time: Cell<f32>,
    pub current_frame: Cell<u64>,
    pub mouse_pos: Cell<Pos2>,

    pub repaint: RefCell<super::Repaint>,
    pub animation: RefCell<animation::Manager>,
    pub commands: RefCell<VecDeque<Command>>,

    pub debug: RefCell<Vec<String>>,
    pub quit: Cell<bool>,
}

impl Inner {
    pub fn new(rect: impl Into<Rect>) -> Self {
        let rect = rect.into();

        let mut nodes = SlotMap::with_key();
        Self {
            root: nodes.insert(Node {
                widget: Box::new(RootWidget),
                parent: None,
                children: Vec::new(),
                next: 0,
            }),
            nodes: RefCell::new(nodes),
            rect: Cell::new(rect),
            ..Self::default()
        }
    }

    pub fn begin(ui: &Ui) -> std::io::Result<()> {
        context::bind(ui);

        let this = &ui.inner;
        this.nodes.borrow_mut()[this.root].next = 0;
        this.input.borrow_mut().start();

        Ok(())
    }

    pub fn end(ui: &Ui) {
        let this = &ui.inner;

        this.removed.borrow_mut().clear();

        Self::cleanup(
            &mut this.nodes.borrow_mut(), //
            &mut this.removed.borrow_mut(),
            this.root,
        );

        // TODO clean up animations here

        let mut input = this.input.borrow_mut();
        for removed in &*this.removed.borrow() {
            input.remove(*removed);
        }
        input.end();

        this.clip_stack.borrow_mut().clear();

        let (mut mouse, mut keyboard) =
            RefMut::map_split(input, |input| (&mut input.mouse, &mut input.keyboard));

        let mut layout = Layout {
            nodes: &this.nodes.borrow(),
            computed: &mut this.computed.borrow_mut(),
            stack: &mut this.stack.borrow_mut(),
            mouse: &mut mouse,
            keyboard: &mut keyboard,
            clip_stack: &mut this.clip_stack.borrow_mut(),
        };

        let mut ctx = LayoutCtx {
            current: this.root,
            children: &layout.nodes[this.root].children,
            layout: &mut layout,
        };

        let value = this.rect.get().size() + Vec2::splat(1);
        ctx.compute(this.root, Constraints::tight(value.into()));

        Self::resolve(this.root, layout.nodes, layout.computed);

        context::unbind();
    }

    pub fn paint(&self, surface: &mut Surface) {
        let mut paint = Paint::default();
        for debug in self.debug.borrow_mut().drain(..) {
            paint.debug(debug);
        }
        paint.paint_all(self, &mut surface.crop(self.rect.get()));
    }
}

impl Inner {
    pub fn available_rect(&self) -> Rect {
        let id = self.current();
        self.computed
            .borrow()
            .get(id)
            .map(|n| n.rect)
            .unwrap_or_default()
    }

    pub fn client_rect(&self) -> Rect {
        self.rect.get()
    }

    pub fn remaining(&self, clock: Instant) -> Duration {
        self.repaint.borrow().remaining(clock)
    }

    pub fn tick(&self, t: f32) {
        let cr = self.current_frame.get();
        self.current_frame.set(cr + 1);
        self.time.set(t);

        let time = Duration::from_secs_f32(self.time.get());
        self.animation.borrow_mut().tick(time)
    }

    pub fn command(&self, cmd: Command) {
        self.commands.borrow_mut().push_back(cmd)
    }

    pub fn debug(&self, debug: impl ToString) {
        self.debug.borrow_mut().push(debug.to_string())
    }

    pub fn handle_external_commands(&self, terminal: &mut Terminal) -> std::io::Result<()> {
        for cmd in self.commands.borrow_mut().drain(..) {
            match cmd {
                Command::SetTitle(title) => terminal.set_title(&title)?,
                Command::Quit => self.quit.set(true),
                Command::LeaveAltScreen => terminal.leave_alt_screen()?,
                Command::EnterAltScreen => terminal.enter_alt_screen()?,
            }
        }
        Ok(())
    }
}

impl Inner {
    pub fn handle_event(&self, event: &Event) -> bool {
        if let &Event::Resize(rect) = event {
            self.rect.set(rect);
            return true;
        }

        let resp = self.input.borrow_mut().handle(
            event, //
            &mut self.nodes.borrow_mut(),
            &mut self.computed.borrow_mut(),
        );

        self.mouse_pos.set(self.input.borrow().mouse.pos);
        resp == Handled::Sink
    }

    pub fn root(&self) -> WidgetId {
        self.root
    }

    pub fn layout_node(&self, id: WidgetId) -> Ref<'_, LayoutNode> {
        let computed = self.computed.borrow();
        Ref::filter_map(computed, |c| c.get(id)).unwrap()
    }

    pub fn current(&self) -> WidgetId {
        self.stack.borrow().last().copied().unwrap_or(self.root())
    }

    pub fn request_repaint(&self) {
        self.request_repaint_after(Duration::ZERO)
    }

    pub fn request_repaint_after(&self, after: Duration) {
        self.repaint.borrow_mut().request_repaint_after(after)
    }

    pub fn time(&self) -> Duration {
        Duration::from_secs_f32(self.time.get())
    }

    pub fn current_frame(&self) -> u64 {
        self.current_frame.get()
    }

    pub fn mouse_over(&self) -> bool {
        let Some(&id) = self.stack.borrow().last() else {
            return false;
        };
        self.mouse_over_widget(id)
    }

    pub fn mouse_over_widget(&self, id: WidgetId) -> bool {
        self.input.borrow().mouse.mouse_over.contains(&id)
    }

    pub fn animate_bool(
        &self,
        source: impl std::hash::Hash,
        value: bool,
        animation_time: f32,
    ) -> f32 {
        self.animation.borrow_mut().animate_bool(
            animation::Id::new(source),
            self.time(),
            value,
            animation_time,
        )
    }

    pub fn animate_value(
        &self,
        source: impl std::hash::Hash,
        value: f32,
        animation_time: f32,
    ) -> f32 {
        self.animation
            .borrow_mut()
            .animate_value(animation::Id::new(source), value, animation_time)
    }

    pub fn begin_widget<W: Widget>(&self, props: W::Props<'_>) -> Response<W::Response> {
        let parent = self.current();
        let (id, mut widget) = self.update_widget::<W>(parent);

        self.stack.borrow_mut().push(id);
        let resp = {
            let Some(widget) = widget.as_any_mut().downcast_mut::<W>() else {
                unreachable!("expected to get: {}", widget.type_name())
            };
            widget.update(props) // FIXME pass in the UI here
        };

        self.nodes.borrow_mut()[id].widget = widget;
        Response::new(id, resp)
    }

    pub fn end_widget(&self, id: WidgetId) {
        let Some(old) = self.stack.borrow_mut().pop() else {
            unreachable!("called end widget without an active widget")
        };
        assert_eq!(id, old, "end widget did not match input widget");

        Self::cleanup(
            &mut self.nodes.borrow_mut(),
            &mut self.removed.borrow_mut(),
            id,
        );
    }

    pub fn remove_all_widgets(&self) {
        let mut nodes = self.nodes.borrow_mut();
        let root = &mut nodes[self.root];
        root.next = 0;
        root.children.clear();
        Self::cleanup(&mut nodes, &mut self.removed.borrow_mut(), self.root)
    }
}

impl Inner {
    fn update_widget<W: Widget>(&self, parent: WidgetId) -> (WidgetId, Box<dyn ErasedWidget>) {
        let mut nodes = self.nodes.borrow_mut();

        let Some(id) = Self::append_widget(&mut nodes, parent) else {
            return Self::allocate_widget::<W>(&mut nodes, parent);
        };

        let Some(node) = nodes.get_mut(id) else {
            unreachable!("node {id:?} must exist")
        };

        let widget = std::mem::replace(&mut node.widget, Box::new(PlaceholderWidget));
        if widget.as_ref().type_id() != TypeId::of::<W>() {
            Self::remove_widget(&mut nodes, &mut self.removed.borrow_mut(), id);
            return Self::allocate_widget::<W>(&mut nodes, parent);
        }

        node.next = 0;
        (id, widget)
    }

    fn append_widget(nodes: &mut SlotMap<WidgetId, Node>, id: WidgetId) -> Option<WidgetId> {
        let parent = &mut nodes[id];
        let &id = parent.children.get(parent.next)?;
        parent.next += 1;
        Some(id)
    }

    fn allocate_widget<W: Widget>(
        nodes: &mut SlotMap<WidgetId, Node>,
        parent: WidgetId,
    ) -> (WidgetId, Box<dyn ErasedWidget>) {
        let id = nodes.insert(Node {
            widget: Box::new(PlaceholderWidget),
            parent: Some(parent),
            children: Vec::new(),
            next: 0,
        });

        let parent = &mut nodes[parent];
        if parent.next < parent.children.len() {
            parent.children[parent.next] = id;
        } else {
            parent.children.push(id)
        }
        parent.next += 1;
        (id, <Box<W>>::default() as Box<dyn ErasedWidget>)
    }

    fn remove_widget(
        nodes: &mut SlotMap<WidgetId, Node>,
        removed: &mut Vec<WidgetId>,
        id: WidgetId,
    ) {
        let mut queue = VecDeque::from_iter([id]);
        while let Some(id) = queue.pop_front() {
            removed.push(id);

            if let Some(node) = nodes.remove(id) {
                queue.extend(node.children());
                if let Some(parent) = node.parent {
                    if let Some(parent) = nodes.get_mut(parent) {
                        parent.children.retain(|&child| child != id)
                    }
                }
            }
        }
    }

    fn cleanup(nodes: &mut SlotMap<WidgetId, Node>, removed: &mut Vec<WidgetId>, start: WidgetId) {
        let node = &mut nodes[start];
        if node.next >= node.children.len() {
            return;
        }

        let children = &node.children[node.next..];
        let mut queue = VecDeque::from_iter(children.iter().copied());
        removed.extend_from_slice(children);
        node.children.truncate(node.next);

        while let Some(id) = queue.pop_front() {
            removed.push(id);
            let Some(next) = nodes.remove(id) else {
                unreachable!("child: {id:?} should exist for {start:?}")
            };
            queue.extend(next.children())
        }
    }

    fn resolve(
        root: WidgetId,
        nodes: &SlotMap<WidgetId, Node>,
        computed: &mut SecondaryMap<WidgetId, LayoutNode>,
    ) {
        let mut queue = VecDeque::new();
        queue.push_back((root, Pos2::ZERO));

        while let Some((id, pos)) = queue.pop_front() {
            let Some(node) = computed.get_mut(id) else {
                continue;
            };

            node.rect = node.rect.translate(pos.to_vec2());
            queue.extend(nodes[id].children().iter().map(|&id| (id, node.rect.min)));
        }
    }
}
