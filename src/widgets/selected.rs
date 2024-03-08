use crate::{
    color::Rgba,
    paint::{Attribute, Label, Styled},
    ui,
};

use super::{filled, label, on_click, List};

#[derive(Debug)]
pub struct Radio {
    pub active_fill: Rgba,
    pub inactive_fill: Option<Rgba>,
}

impl Radio {
    pub fn new(active_fill: impl Into<Rgba>) -> Self {
        Self {
            active_fill: active_fill.into(),
            inactive_fill: None,
        }
    }

    pub fn inactive_fill(mut self, inactive_fill: impl Into<Rgba>) -> Self {
        self.inactive_fill = Some(inactive_fill.into());
        self
    }

    pub fn show<R, V: PartialEq>(self, value: &mut V, selected: V, show: impl FnOnce() -> R) {
        let resp = on_click(|| {
            let bg = if *value == selected {
                self.active_fill
            } else {
                self.inactive_fill.unwrap_or(self.active_fill.darken(0.3))
            };
            filled(bg, show)
        });

        if resp.clicked {
            *value = selected
        }
    }
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            active_fill: Rgba::from_u32(0x4169E1),
            inactive_fill: Some(Rgba::from_u32(0x425057)),
        }
    }
}

pub fn radio<R, V: PartialEq>(value: &mut V, selected: V, show: impl FnOnce() -> R) {
    Radio::default().show(value, selected, show)
}

#[derive(Copy, Clone, Debug)]
pub struct Checkbox {
    pub selected: char,   // these could be labels
    pub unselected: char, // these could be labels
}

impl Checkbox {
    pub fn show<R>(self, value: &mut bool, show: impl FnOnce() -> R) {
        let resp = on_click(|| {
            List::row().spacing(1).show(|| {
                let button = if *value { '☒' } else { '☐' };
                label(button);
                show();
            })
        });
        *value ^= resp.clicked
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            selected: '☒',
            unselected: '☐',
        }
    }
}

pub fn checkbox<R>(value: &mut bool, show: impl FnOnce() -> R) {
    Checkbox::default().show(value, show)
}

// this needs a mouseover
#[derive(Copy, Clone, Debug, Default)]
pub struct TodoValue;

impl TodoValue {
    pub fn show<L: Label>(self, value: &mut bool, show: impl Into<Styled<L>>) {
        let resp = on_click(|| {
            let ui = ui();
            let mut show = show.into();
            let hovered = ui.mouse_over_widget(ui.current());
            show = if hovered { show.fg(0xFF00FF) } else { show };

            if *value {
                let attribute = Attribute::STRIKEOUT | Attribute::ITALIC | Attribute::FAINT;
                label(show.attr(attribute))
            } else {
                label(show)
            }
        });

        *value ^= resp.clicked
    }
}

pub fn todo_value<L: Label>(value: &mut bool, show: impl Into<Styled<L>>) {
    TodoValue.show(value, show)
}

#[derive(Copy, Clone, Debug)]
pub struct Selected {
    active_fill: Rgba,
    inactive_fill: Option<Rgba>,
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            active_fill: Rgba::from_u32(0x4169E1),
            inactive_fill: Some(Rgba::from_u32(0x425057)),
        }
    }
}

impl Selected {
    pub fn show<R>(self, value: &mut bool, show: impl FnOnce() -> R) {
        let resp = on_click(|| {
            let bg = if *value {
                self.active_fill
            } else {
                self.inactive_fill.unwrap_or(self.active_fill.darken(0.3))
            };
            filled(bg, show)
        });
        *value ^= resp.clicked
    }
}

pub fn selected<R>(value: &mut bool, show: impl FnOnce() -> R) {
    Selected::default().show(value, show)
}
