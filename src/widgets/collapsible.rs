use crate::{
    color::Rgba,
    paint::{Attribute, Label, Styled},
    ui,
};

use super::{column, filled, label, on_click, List};

pub fn collapsible<R, T: Label>(
    state: &mut bool,
    title: impl Into<Styled<T>>,
    show: impl FnOnce() -> R,
) {
    let ui = ui();

    column(|| {
        let resp = on_click(|| {
            let hovered = ui.mouse_over();
            filled(Rgba::from_u32(0x333333).with_alpha(0xAA), || {
                List::row().spacing(1).show(|| {
                    let mut icon = Styled::new(if *state { '▼' } else { '▶' });
                    if hovered {
                        icon = icon.attr(Attribute::BOLD);
                    }

                    label(icon);

                    let mut title = title.into();
                    if hovered {
                        title = title.attr(Attribute::BOLD);
                    }

                    label(title);
                });
            });
        });

        *state ^= resp.clicked;
        if *state {
            show();
        }
    });
}
