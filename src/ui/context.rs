use std::cell::RefCell;

use crate::ui::Ui;

thread_local! {
    static CURRENT: RefCell<Option<Ui>> = const { RefCell::new(None) }
}

pub(crate) fn bind(ui: &Ui) {
    CURRENT.with(move |current| {
        let mut current = current.borrow_mut();
        assert!(current.is_none(), "ui is already bound to this thread");
        *current = Some(ui.clone())
    })
}

pub(crate) fn unbind() {
    CURRENT.with(|current| {
        let mut current = current.borrow_mut();
        assert!(current.take().is_some(), "ui was not bound to this thread")
    })
}

pub(crate) fn current() -> Ui {
    CURRENT.with(|current| {
        current
            .borrow()
            .as_ref()
            .expect("cannot get access to ui without one bound")
            .clone()
    })
}
