use std::time::{Duration, Instant};

pub mod animation;
pub mod color;
pub mod context;
pub mod geom;
pub mod input;
pub mod paint;
pub mod widgets;

mod node;
pub use node::{LayoutNode, Node, WidgetId};

mod queue;
pub use queue::Queue;

mod terminal;
pub use terminal::Config;
use terminal::Terminal;

mod ui;
pub use ui::{debug, ui, Command, Ui};

mod widget;
pub use widget::{NoResponse, Widget, WidgetExt};

mod debug_fmt;

pub fn run<R>(config: Config, mut app: impl FnMut(&Ui) -> R) -> std::io::Result<()> {
    let limited_rate = Duration::from_secs_f32(1.0 / 10.0);

    let mut terminal = Terminal::new(config)?;
    let ui = Ui::new(terminal.rect());

    terminal.set_title(format!(
        "{}x{} ({})",
        terminal.rect().width(),
        terminal.rect().height(),
        terminal.rect().area()
    ))?;

    let start = Instant::now();

    while !ui.quit() {
        let clock = Instant::now();

        while let Some(ev) = terminal.read_next_event() {
            if matches!(ev, terminal::event::Event::Quit) {
                ui.set_quit();
                // TODO let the app also get the quit event
                if ui.quit() {
                    break;
                }
            }

            // TODO handle this
            let _skip = ui.handle_event(&ev);
            ui.handle_external_commands(&mut terminal)?;
        }

        ui.scope(|| app(&ui))?;

        if terminal.is_in_alt_screen() {
            terminal.paint(|canvas| {
                canvas.erase();
                ui.paint(canvas)
            })?;
        }

        if ui.quit() {
            break;
        }

        ui.tick(start.elapsed().as_secs_f32());
        std::thread::sleep(ui.remaining(clock).min(limited_rate));
    }

    Ok(())
}
