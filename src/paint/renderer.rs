use std::fmt::Write as _;

use super::Attribute;
use crate::{color::Rgba, geom::Pos2};

pub trait Renderer {
    fn begin(&mut self) -> std::io::Result<()>;
    fn end(&mut self) -> std::io::Result<()>;

    fn move_to(&mut self, pos: Pos2) -> std::io::Result<()>;
    fn write(&mut self, ch: char) -> std::io::Result<()>;

    fn set_fg(&mut self, rgb: Rgba) -> std::io::Result<()>;
    fn set_bg(&mut self, rgb: Rgba) -> std::io::Result<()>;
    fn set_attr(&mut self, attr: Attribute) -> std::io::Result<()>;

    fn reset_fg(&mut self) -> std::io::Result<()>;
    fn reset_bg(&mut self) -> std::io::Result<()>;
    fn reset_attr(&mut self) -> std::io::Result<()>;

    fn capture_mouse(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn release_mouse(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn enter_alt_screen(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn leave_alt_screen(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn enable_line_wrap(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn disable_line_wrap(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn set_title(&mut self, title: &str) -> std::io::Result<()> {
        let _ = title;
        Ok(())
    }
}

pub struct TermRenderer<W> {
    out: W,
}

impl<W> TermRenderer<W> {
    pub const fn new(out: W) -> Self {
        Self { out }
    }
}

macro_rules! csi { ($($lit:literal),*) => { concat!( $("\x1b[",$lit),*).as_bytes() };}

impl<W: std::io::Write> Renderer for TermRenderer<W> {
    fn begin(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?2026h"))
    }

    fn end(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?2026l"))?;
        self.out.flush()
    }

    fn move_to(&mut self, pos: Pos2) -> std::io::Result<()> {
        let (x, y) = (pos.x + 1, pos.y + 1);
        self.out.write_fmt(format_args!("\x1b[{y};{x};H",))
    }

    fn write(&mut self, ch: char) -> std::io::Result<()> {
        self.out.write_all(ch.encode_utf8(&mut [0; 4]).as_bytes())
    }

    fn set_fg(&mut self, Rgba(r, g, b, ..): Rgba) -> std::io::Result<()> {
        self.out.write_fmt(format_args!("\x1b[38;2;{r};{g};{b}m"))
    }

    fn set_bg(&mut self, Rgba(r, g, b, ..): Rgba) -> std::io::Result<()> {
        self.out.write_fmt(format_args!("\x1b[48;2;{r};{g};{b}m"))
    }

    fn set_attr(&mut self, attr: Attribute) -> std::io::Result<()> {
        [
            attr.is_reset(),
            attr.is_bold(),
            attr.is_faint(),
            attr.is_italic(),
            attr.is_underline(),
            attr.is_blink(),
            false, // placeholder
            attr.is_reverse(),
            false, // placeholder
            attr.is_strikeout(),
        ]
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c)
        .try_for_each(|(n, _)| self.out.write_fmt(format_args!("\x1b[{n}m")))
    }

    fn reset_fg(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("39m"))
    }

    fn reset_bg(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("49m"))
    }

    fn reset_attr(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("0m"))
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?25h"))
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?25l"))
    }

    fn capture_mouse(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!(
            "?1000h", //
            "?1002h", //
            "?1003h", //
            "?1006h", //
            "?1015h"
        ))
    }

    fn release_mouse(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!(
            "?1000l", //
            "?1002l", //
            "?1003l", //
            "?1006l", //
            "?1015l"
        ))
    }

    fn enter_alt_screen(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?1048h"))
    }

    fn leave_alt_screen(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?1048l"))
    }

    fn enable_line_wrap(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?7h"))
    }

    fn disable_line_wrap(&mut self) -> std::io::Result<()> {
        self.out.write_all(csi!("?7l"))
    }

    fn set_title(&mut self, title: &str) -> std::io::Result<()> {
        self.out.write_fmt(format_args!("\x1b]2;{title}\x07"))
    }
}

#[derive(Default)]
pub struct DebugRenderer {
    pub out: String,
    incomplete: bool,
}

impl DebugRenderer {
    fn next_entry(&mut self) {
        if self.incomplete {
            self.out.push('\n');
            self.incomplete = !self.incomplete
        }
    }
}

impl std::fmt::Display for DebugRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.out.fmt(f)
    }
}

impl Renderer for DebugRenderer {
    fn begin(&mut self) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "begin");
        Ok(())
    }

    fn end(&mut self) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "end");
        Ok(())
    }

    fn move_to(&mut self, pos: Pos2) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  move_to {pos:?}");
        Ok(())
    }

    fn write(&mut self, ch: char) -> std::io::Result<()> {
        if !self.incomplete {
            self.out.push_str("    ");
        }
        self.incomplete = true;
        let ch = match ch {
            ' ' => '░',
            d => d,
        };

        let _ = write!(&mut self.out, "{ch}");
        Ok(())
    }

    fn set_fg(&mut self, rgb: Rgba) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  set_fg: {rgb:?}");
        Ok(())
    }

    fn set_bg(&mut self, rgb: Rgba) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  set_bg: {rgb:?}");
        Ok(())
    }

    fn set_attr(&mut self, attr: Attribute) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  set_attr: {attr:?}");
        Ok(())
    }

    fn reset_fg(&mut self) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  reset_fg");
        Ok(())
    }

    fn reset_bg(&mut self) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  reset_bg");
        Ok(())
    }

    fn reset_attr(&mut self) -> std::io::Result<()> {
        self.next_entry();
        let _ = writeln!(&mut self.out, "  reset_attr");
        Ok(())
    }
}
