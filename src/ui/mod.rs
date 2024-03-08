use crate::geom::{Pos2, Rect, Vec2};
use crate::terminal::event::Event;
use crate::widget::Response;
use crate::{
    animation, debug_fmt,
    node::{LayoutNode, Node, WidgetId},
    widget::Widget,
};
use crate::{paint::Surface, terminal::Terminal};

use slotmap::{SecondaryMap, SlotMap};
use std::{
    cell::{Ref, RefMut},
    rc::Rc,
    time::{Duration, Instant},
};

mod layout;
pub(crate) use layout::Layout;

mod paint;
pub(crate) use paint::Paint;

mod context;
pub mod debug;

mod repaint;
use repaint::Repaint;

mod inner;
pub(crate) use inner::Inner;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Command {
    SetTitle(String),
    LeaveAltScreen,
    EnterAltScreen,
    Quit,
}

pub fn ui() -> Ui {
    context::current()
}

pub struct Ui {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for Ui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = &self.inner;
        f.debug_struct("Ui")
            .field("nodes", &debug_fmt::slot_map(&this.nodes.borrow()))
            .field(
                "computed",
                &debug_fmt::secondary_map(&this.computed.borrow()),
            )
            .field("clip_stack", &debug_fmt::vec(&this.clip_stack.borrow()))
            .field("input", &this.input.borrow())
            .field("rect", &this.rect)
            .finish()
    }
}

impl Ui {
    pub fn command(&self, cmd: Command) {
        self.inner.command(cmd)
    }

    pub fn set_title(&self, title: impl ToString) {
        self.command(Command::SetTitle(title.to_string()))
    }

    pub fn debug(&self, debug: impl ToString) {
        self.inner.debug(debug)
    }
}

impl Ui {
    pub fn size(&self) -> Vec2 {
        self.client_rect().size()
    }

    pub fn available_rect(&self) -> Rect {
        self.inner.available_rect()
    }

    pub fn client_rect(&self) -> Rect {
        self.inner.client_rect()
    }

    pub fn current_frame(&self) -> u64 {
        self.inner.current_frame()
    }

    pub fn time(&self) -> Duration {
        Duration::from_secs_f32(self.inner.time.get())
    }

    pub fn mouse_pos(&self) -> Pos2 {
        self.inner.mouse_pos.get()
    }
}

impl Ui {
    pub fn widget<W: Widget>(&self, props: W::Props<'_>) -> Response<W::Response> {
        let resp = self.inner.begin_widget::<W>(props);
        self.inner.end_widget(resp.id());
        resp
    }

    pub fn root(&self) -> WidgetId {
        self.inner.root
    }

    pub fn current(&self) -> WidgetId {
        self.inner.current()
    }

    pub fn layout_node(&self, id: WidgetId) -> Ref<'_, LayoutNode> {
        let computed = self.inner.computed.borrow();
        Ref::filter_map(computed, |c| c.get(id)).unwrap()
    }

    pub fn get(&self, id: WidgetId) -> Option<Ref<'_, Node>> {
        let nodes = self.inner.nodes.borrow();
        Ref::filter_map(nodes, |nodes| nodes.get(id)).ok()
    }

    pub fn get_mut(&self, id: WidgetId) -> Option<RefMut<'_, Node>> {
        let nodes = self.inner.nodes.borrow_mut();
        RefMut::filter_map(nodes, |nodes| nodes.get_mut(id)).ok()
    }

    pub fn get_current(&self) -> Ref<'_, Node> {
        self.get(self.current()).unwrap()
    }

    pub fn remove_all_widgets(&self) {
        self.inner.remove_all_widgets()
    }
}

impl Ui {
    pub fn mouse_over(&self) -> bool {
        self.inner.mouse_over()
    }

    pub fn mouse_over_widget(&self, id: WidgetId) -> bool {
        self.inner.mouse_over_widget(id)
    }

    pub fn request_repaint(&self) {
        self.request_repaint_after(Duration::ZERO)
    }

    pub fn request_repaint_after(&self, after: Duration) {
        self.inner.request_repaint_after(after)
    }

    pub fn animate_bool(
        &self,
        source: impl std::hash::Hash,
        value: bool,
        animation_time: f32,
    ) -> f32 {
        self.inner.animate_bool(source, value, animation_time)
    }

    pub fn animate_value(
        &self,
        source: impl std::hash::Hash,
        value: f32,
        animation_time: f32,
    ) -> f32 {
        self.inner
            .animate_value(animation::Id::new(source), value, animation_time)
    }
}

impl Ui {
    pub(crate) fn new(rect: impl Into<Rect>) -> Self {
        Self {
            inner: Rc::new(Inner::new(rect)),
        }
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }

    pub(crate) fn scope<R>(&self, f: impl FnOnce() -> R) -> std::io::Result<R> {
        Inner::begin(self)?;
        let resp = f();
        Inner::end(self);
        Ok(resp)
    }

    // TODO these are not ideal
    pub(crate) fn set_quit(&self) {
        self.inner.quit.set(true)
    }

    // TODO these are not ideal
    pub(crate) fn quit(&self) -> bool {
        self.inner.quit.get()
    }

    pub(crate) fn nodes(&self) -> Ref<'_, SlotMap<WidgetId, Node>> {
        self.inner.nodes.borrow()
    }

    pub(crate) fn computed(&self) -> Ref<'_, SecondaryMap<WidgetId, LayoutNode>> {
        self.inner.computed.borrow()
    }

    pub(crate) fn begin_widget<W: Widget>(&self, props: W::Props<'_>) -> Response<W::Response> {
        self.inner.begin_widget::<W>(props)
    }

    pub(crate) fn end_widget(&self, id: WidgetId) {
        self.inner.end_widget(id)
    }

    pub(crate) fn remaining(&self, clock: Instant) -> Duration {
        self.inner.remaining(clock)
    }

    pub(crate) fn tick(&self, t: f32) {
        self.inner.tick(t)
    }

    pub(crate) fn handle_event(&self, event: &Event) -> bool {
        self.inner.handle_event(event)
    }

    pub(crate) fn handle_external_commands(&self, terminal: &mut Terminal) -> std::io::Result<()> {
        self.inner.handle_external_commands(terminal)
    }

    pub(crate) fn paint(&self, surface: &mut Surface) {
        self.inner.paint(surface)
    }
}
