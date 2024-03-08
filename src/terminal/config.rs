use std::sync::{Arc, RwLock};

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub(crate) hide_cursor: bool,
    pub(crate) mouse_capture: bool,
    pub(crate) ctrl_c_quits: bool,
    pub(crate) ctrl_z_switches: bool,
    pub(crate) use_alt_screen: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hide_cursor: true,
            mouse_capture: true,
            ctrl_c_quits: true,
            ctrl_z_switches: false,
            use_alt_screen: true,
        }
    }
}

impl Config {
    pub const fn hide_cursor(mut self, hide_cursor: bool) -> Self {
        self.hide_cursor = hide_cursor;
        self
    }

    pub const fn mouse_capture(mut self, mouse_capture: bool) -> Self {
        self.mouse_capture = mouse_capture;
        self
    }

    pub const fn ctrl_c_quits(mut self, ctrl_c_quits: bool) -> Self {
        self.ctrl_c_quits = ctrl_c_quits;
        self
    }

    pub const fn ctrl_z_switches(mut self, ctrl_z_switches: bool) -> Self {
        self.ctrl_z_switches = ctrl_z_switches;
        self
    }

    pub const fn use_alt_screen(mut self, use_alt_screen: bool) -> Self {
        self.use_alt_screen = use_alt_screen;
        self
    }

    pub fn into_shareable(self) -> ShareableConfig {
        self.into()
    }
}

#[derive(Clone)]
pub struct ShareableConfig {
    inner: Arc<RwLock<Config>>,
}

impl From<Config> for ShareableConfig {
    fn from(value: Config) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }
}

impl ShareableConfig {
    pub fn mutate(&self, mut f: impl FnMut(&mut Config)) {
        f(&mut self.inner.write().unwrap())
    }

    pub fn get<T>(&self, f: impl Fn(&Config) -> T) -> T {
        f(&self.inner.read().unwrap())
    }
}
