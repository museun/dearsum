use crate::{
    geom::{rect, vec2, Rect, Vec2},
    paint::{Renderer as _, Surface, TermRenderer},
};
use std::io::{BufWriter, Write as _};

pub mod event;
use self::event::{read_next_event, Event, Key, Modifiers, MouseState};

mod config;
pub use config::Config;
use config::ShareableConfig;

pub struct Terminal {
    config: ShareableConfig,
    out: BufWriter<std::io::Stdout>,
    mouse: MouseState,
    surface: Surface,
    _guard: Guard,
}

impl Terminal {
    pub fn new(config: Config) -> std::io::Result<Self> {
        let (rect, out, _guard, config) = Self::setup(config)?;
        Self::install_panic_hook(config.clone());

        Ok(Self {
            config,
            mouse: MouseState::default(),
            surface: Surface::new(rect.size()),
            out: BufWriter::with_capacity((rect.area() as usize * 21).next_power_of_two(), out),
            _guard,
        })
    }

    pub fn rect(&self) -> Rect {
        self.surface.current().rect()
    }

    pub fn read_next_event(&mut self) -> Option<Event> {
        let ev = read_next_event(&mut self.mouse)?;

        match ev {
            Event::Keyboard(Key::Char('c'), Modifiers::CTRL) => {
                if self.config.get(|c| c.ctrl_c_quits) {
                    return Some(Event::Quit);
                }
            }
            Event::Resize(rect) => self.resize(rect.size()),
            _ => {}
        }

        Some(ev)
    }

    pub fn paint(&mut self, mut draw: impl FnMut(&mut Surface)) -> std::io::Result<()> {
        draw(&mut self.surface);
        self.surface.render(&mut TermRenderer::new(&mut self.out))
    }

    fn resize(&mut self, size: Vec2) {
        let (out, _) = std::mem::replace(
            &mut self.out,
            BufWriter::with_capacity(0, std::io::stdout()),
        )
        .into_parts();

        let cap = size.x as usize * size.y as usize * 21;
        self.out = BufWriter::with_capacity(cap.next_power_of_two(), out);
        self.surface.resize(size)
    }
}

impl Terminal {
    pub fn is_in_alt_screen(&self) -> bool {
        self.config.get(|c| c.use_alt_screen)
    }

    pub fn set_title(&self, title: impl AsRef<str>) -> std::io::Result<()> {
        self.immediate(|mut p| p.set_title(title.as_ref()))
    }

    pub fn enter_alt_screen(&self) -> std::io::Result<()> {
        if self.is_in_alt_screen() {
            return Ok(());
        }

        self.config.mutate(|c| c.use_alt_screen = true);
        self.immediate(|mut p| {
            p.enter_alt_screen()?;
            p.disable_line_wrap()?;
            p.capture_mouse()
        })
    }

    pub fn leave_alt_screen(&self) -> std::io::Result<()> {
        if !self.is_in_alt_screen() {
            return Ok(());
        }

        self.config.mutate(|c| c.use_alt_screen = false);
        self.immediate(|mut p| {
            p.leave_alt_screen()?;
            p.release_mouse()?;
            p.enable_line_wrap()
        })
    }

    fn immediate<F>(&self, apply: F) -> std::io::Result<()>
    where
        F: Fn(TermRenderer<&std::io::Stdout>) -> std::io::Result<()>,
    {
        let mut out = self.out.get_ref();
        apply(TermRenderer::new(out))?;
        out.flush()
    }
}

impl Terminal {
    fn setup(config: Config) -> std::io::Result<(Rect, std::io::Stdout, Guard, ShareableConfig)> {
        use crossterm::terminal;

        let mut out = std::io::stdout();

        // TOOD get rid of these and we get rid of crossterm from here
        terminal::enable_raw_mode()?;

        if config.use_alt_screen {
            crossterm::execute!(&mut out, crossterm::terminal::EnterAlternateScreen)?;
            crossterm::execute!(&mut out, crossterm::terminal::DisableLineWrap)?;
        }

        if config.hide_cursor {
            crossterm::execute!(&mut out, crossterm::cursor::Hide)?;
        }

        if config.mouse_capture {
            crossterm::execute!(&mut out, crossterm::event::EnableMouseCapture)?;
        }

        out.flush()?;

        let config = ShareableConfig::from(config);
        // TOOD get rid of these and we get rid of crossterm from here
        let size = terminal::size().map(|(w, h)| vec2(w as _, h as _))?;

        Ok((rect(size), out, Guard(config.clone()), config))
    }

    fn reset(config: ShareableConfig) -> std::io::Result<()> {
        use crossterm::terminal;
        let mut out = std::io::stdout();

        if config.get(|c| c.use_alt_screen) {
            crossterm::execute!(
                &mut out,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            )?;
            crossterm::execute!(&mut out, crossterm::terminal::LeaveAlternateScreen)?;
            crossterm::execute!(&mut out, crossterm::terminal::EnableLineWrap)?;
        }

        if config.get(|c| c.mouse_capture) {
            crossterm::execute!(&mut out, crossterm::event::DisableMouseCapture)?;
        }
        crossterm::execute!(&mut out, crossterm::cursor::Show)?;

        out.flush()?;
        // TODO get rid of this
        terminal::disable_raw_mode()
    }

    fn install_panic_hook(config: ShareableConfig) {
        let old = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = Self::reset(config.clone());
            old(info)
        }));
    }
}

struct Guard(ShareableConfig);

impl Drop for Guard {
    fn drop(&mut self) {
        let _ = Terminal::reset(self.0.clone());
    }
}
