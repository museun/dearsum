use slotmap::{SecondaryMap, SlotMap};
use std::collections::HashSet;

use crate::context::EventCtx;
use crate::geom::{Pos2, Vec2};
use crate::node::{LayoutNode, Node, WidgetId};
use crate::terminal::event::{Event as CoreEvent, MouseEvent};

pub use crate::terminal::event::{Key, Keybind, Modifiers, MouseButton};

mod events;
pub use events::{Event, KeyPressed, MouseClick, MouseDrag, MouseHeld, MouseMove, MouseScroll};

mod mouse;
use mouse::ButtonState;
pub(crate) use mouse::Mouse;

mod keyboard;
pub(crate) use keyboard::Keyboard;

mod interest;
pub use interest::Interest;

mod layered;
use layered::Layered;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum Handled {
    Sink,
    #[default]
    Bubble,
}

impl Handled {
    const fn is_sink(&self) -> bool {
        matches!(self, Self::Sink)
    }
}

#[derive(Default, Debug)]
struct Intersections {
    hit: HashSet<WidgetId>,
    entered: Vec<WidgetId>,
    entered_and_sunk: Vec<WidgetId>,
}

#[derive(Debug, Default)]
pub(crate) struct Input {
    pub(crate) mouse: Mouse,
    pub(crate) keyboard: Keyboard,
    modifiers: Modifiers,
    intersections: Intersections,
    // focus: Option<WidgetId>,
    // last_focus: Option<WidgetId>,
    last_event: Option<CoreEvent>,
}

impl Input {
    pub(crate) fn start(&mut self) {
        // notify focus
    }

    pub(crate) fn end(&mut self) {
        // interpolate

        self.keyboard.clear();
        self.mouse.clear();
    }

    pub(crate) fn handle(
        &mut self,
        event: &CoreEvent,
        nodes: &mut SlotMap<WidgetId, Node>,
        layout: &mut SecondaryMap<WidgetId, LayoutNode>,
        debug: &mut Vec<String>,
    ) -> Handled {
        self.last_event = Some(event.clone());
        match *event {
            CoreEvent::Mouse(event, pos, modifiers) => {
                self.modifiers = modifiers;
                self.mouse_event(event, pos, nodes, layout, debug)
            }
            CoreEvent::Keyboard(key, modifiers) => {
                self.modifiers = modifiers;
                let event = KeyPressed {
                    key,
                    modifiers: self.modifiers,
                };

                let mut resp = Handled::Bubble;
                for (id, ()) in self.keyboard.layered.iter() {
                    if resp.is_sink() {
                        break;
                    }

                    let node = &mut nodes[*id];
                    let ctx = EventCtx {
                        rect: layout[*id].rect,
                        current: *id,
                        children: &node.children,
                        hovered: &self.mouse.mouse_over,
                        computed: layout,
                        debug,
                    };

                    resp = node.widget.event(ctx, Event::KeyInput(event));
                }

                resp
            }
            _ => Handled::Bubble,
        }
    }

    fn mouse_event(
        &mut self,
        event: MouseEvent,
        pos: Pos2,
        nodes: &mut SlotMap<WidgetId, Node>,
        layout: &mut SecondaryMap<WidgetId, LayoutNode>,
        debug: &mut Vec<String>,
    ) -> Handled {
        self.mouse.pos = pos;

        macro_rules! ctx {
            () => {
                MouseContext {
                    nodes,
                    layout,
                    mouse: &mut self.mouse,
                    intersections: &mut self.intersections,
                    debug,
                }
            };
        }

        match event {
            MouseEvent::Move { .. } => {
                let event = MouseMove { pos };
                ctx!().mouse_move(event)
            }

            MouseEvent::Click { button } => {
                self.mouse.buttons.insert(button, ButtonState::Released);
                let event = MouseClick {
                    pos,
                    button,
                    modifiers: self.modifiers,
                };
                ctx!().mouse_button(&Event::MouseClick(event))
            }

            MouseEvent::Held { button, .. } => {
                self.mouse.buttons.insert(button, ButtonState::Held);
                let event = MouseHeld {
                    pos,
                    button,
                    modifiers: self.modifiers,
                };
                ctx!().mouse_button(&Event::MouseHeld(event))
            }

            MouseEvent::DragStart { button, .. } => {
                self.mouse.buttons.insert(button, ButtonState::Held);
                let event = MouseDrag {
                    released: false,
                    pos,
                    origin: pos,
                    delta: Vec2::ZERO,
                    button,
                    modifiers: self.modifiers,
                };
                ctx!().mouse_drag(event)
            }

            MouseEvent::DragHeld {
                origin,
                delta,
                button,
            } => {
                self.mouse.buttons.insert(button, ButtonState::Held);
                let event = MouseDrag {
                    released: false,
                    pos,
                    origin,
                    delta,
                    button,
                    modifiers: self.modifiers,
                };
                ctx!().mouse_drag(event)
            }

            MouseEvent::DragRelease { origin, button } => {
                self.mouse.buttons.insert(button, ButtonState::Released);
                let event = MouseDrag {
                    released: true,
                    pos,
                    origin,
                    delta: Vec2::ZERO,
                    button,
                    modifiers: self.modifiers,
                };
                ctx!().mouse_drag(event)
            }

            MouseEvent::Scroll { delta } => {
                let event = MouseScroll {
                    pos,
                    delta,
                    modifiers: self.modifiers,
                };
                ctx!().mouse_scroll(event)
            }
        }
    }

    pub(crate) fn remove(&mut self, removed: WidgetId) {
        self.keyboard.remove(removed);
        self.mouse.remove(removed);
        self.mouse.mouse_over.remove(&removed);
    }
}

struct MouseContext<'a> {
    nodes: &'a mut SlotMap<WidgetId, Node>,
    layout: &'a mut SecondaryMap<WidgetId, LayoutNode>,
    mouse: &'a mut Mouse,

    intersections: &'a mut Intersections,
    debug: &'a mut Vec<String>,
}

impl<'a> MouseContext<'a> {
    fn mouse_move(&mut self, event: MouseMove) -> Handled {
        {
            for (id, interest) in self.mouse.layered.iter() {
                if !interest.is_mouse_move() {
                    continue;
                }

                let node = &mut self.nodes[*id];
                let ctx = EventCtx {
                    rect: self.layout[*id].rect,
                    current: *id,
                    children: &node.children,
                    hovered: &self.mouse.mouse_over,
                    computed: self.layout,
                    debug: self.debug,
                };
                node.widget.event(ctx, Event::MouseMove(event));
            }
        }

        {
            self.intersections.hit.clear();
            self.hit_test(event.pos);
        }
        {
            for &hit in &self.intersections.hit {
                if !self.intersections.entered.contains(&hit) {
                    continue;
                }

                self.intersections.entered.push(hit);

                self.mouse.hovered(hit);

                let node = &mut self.nodes[hit];
                let ctx = EventCtx {
                    rect: self.layout[hit].rect,
                    current: hit,
                    children: &node.children,
                    hovered: &self.mouse.mouse_over,
                    computed: self.layout,
                    debug: self.debug,
                };

                let resp = node.widget.event(ctx, Event::MouseEnter(event));

                if resp.is_sink() {
                    self.intersections.entered_and_sunk.push(hit);
                    break;
                }

                if self.intersections.entered_and_sunk.contains(&hit) {
                    break;
                }
            }
        }

        let mut inactive = vec![];
        {
            for (hit, _) in self.mouse.layered.iter() {
                if !self.intersections.entered.contains(hit) {
                    continue;
                }

                let Some(node) = self.layout.get(*hit) else {
                    continue;
                };

                if node.rect.contains(event.pos) {
                    continue;
                }

                self.mouse.mouse_over.remove(hit);

                let node = &mut self.nodes[*hit];
                let ctx = EventCtx {
                    rect: self.layout[*hit].rect,
                    current: *hit,
                    children: &node.children,
                    hovered: &self.mouse.mouse_over,
                    computed: self.layout,
                    debug: self.debug,
                };
                node.widget.event(ctx, Event::MouseLeave(event));
                inactive.push(hit)
            }
        }
        {
            for inactive in inactive {
                self.intersections.entered.retain(|id| id != inactive);
                self.intersections
                    .entered_and_sunk
                    .retain(|id| id != inactive)
            }
        }

        Handled::Bubble
    }

    fn mouse_button(&mut self, event: &Event) -> Handled {
        let mut resp = Handled::Bubble;

        for (id, interest) in self.mouse.layered.iter() {
            if !interest.is_mouse_any() {
                continue;
            }

            if !self.intersections.hit.contains(id) {
                continue;
            }

            let node = &mut self.nodes[*id];
            let ctx = EventCtx {
                rect: self.layout[*id].rect,
                current: *id,
                children: &node.children,
                hovered: &self.mouse.mouse_over,
                computed: self.layout,
                debug: self.debug,
            };

            resp = node.widget.event(ctx, *event);
            if resp.is_sink() {
                break;
            }
        }

        resp
    }

    fn mouse_drag(&mut self, event: MouseDrag) -> Handled {
        let resp = self.send_mouse_event(&Event::MouseDrag(event));
        if event.released {
            // let mouse_click = MouseClick {
            //     pos: event.pos,
            //     button: event.button,
            //     modifiers: event.modifiers,
            // };
            // self.send_mouse_event(&Event::MouseClick(mouse_click));
        }
        resp
    }

    fn mouse_scroll(&mut self, event: MouseScroll) -> Handled {
        self.send_mouse_event(&Event::MouseScroll(event))
    }

    fn send_mouse_event(&mut self, event: &Event) -> Handled {
        let mut resp = Handled::Bubble;

        for &id in &self.intersections.hit {
            let node = &mut self.nodes[id];
            let ctx = EventCtx {
                rect: self.layout[id].rect,
                current: id,
                children: &node.children,
                hovered: &self.mouse.mouse_over,
                computed: self.layout,
                debug: self.debug,
            };
            resp = node.widget.event(ctx, *event);
            if resp.is_sink() {
                break;
            }
        }

        resp
    }

    fn hit_test(&mut self, pos: Pos2) {
        for (id, _) in self.mouse.layered.iter() {
            let Some(mut node) = self.layout.get(*id) else {
                continue;
            };

            let mut rect = node.rect;
            while let Some(parent) = node.clipped_by {
                node = &self.layout[parent];
                rect = rect.intersection(node.rect);
            }

            if node.rect.contains(pos) {
                self.intersections.hit.insert(*id);
            }
        }
    }
}
