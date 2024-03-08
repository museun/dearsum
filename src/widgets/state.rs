use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{
    context::LayoutCtx,
    geom::{Constraints, Size},
    widget::Response,
    Widget, WidgetExt as _,
};

pub trait Stateful: 'static {}
impl<T: 'static> Stateful for T {}

pub struct StateResponse<T: Stateful> {
    value: Rc<RefCell<T>>,
}

impl<T: Stateful> StateResponse<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }

    pub fn set(&self, value: T) {
        self.value.replace(value);
    }

    pub fn set_if(&self, value: Option<T>) {
        if let Some(value) = value {
            self.set(value)
        }
    }

    pub fn modify(&self, update: impl FnOnce(&mut T)) {
        let mut value = self.borrow_mut();
        update(&mut *value)
    }
}

impl<T: Clone + Stateful> StateResponse<T> {
    pub fn cloned(&self) -> T {
        self.borrow().clone()
    }
}

impl<T: Copy + Stateful> StateResponse<T> {
    pub fn get(&self) -> T {
        *self.borrow()
    }

    pub fn map(&self, update: impl FnOnce(T) -> T) {
        let mut value = self.borrow_mut();
        *value = update(*value)
    }
}

pub struct State<T: Stateful> {
    default: Box<dyn FnOnce() -> T>,
}

impl<T: Stateful> State<T> {
    pub fn new(f: impl FnOnce() -> T + 'static) -> Self {
        Self {
            default: Box::new(f),
        }
    }
}

impl<T: Stateful> std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct Pointer<T: std::fmt::Pointer>(T);
        impl<T: std::fmt::Pointer> std::fmt::Debug for Pointer<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:p}", self.0)
            }
        }

        f.debug_struct("State")
            .field("default", &Pointer(&self.default))
            .finish()
    }
}

pub struct StateWidget<T: Stateful> {
    props: Option<Rc<RefCell<T>>>,
}

impl<T: Stateful> std::fmt::Debug for StateWidget<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateWidget")
            .field("type", &crate::debug_fmt::str(std::any::type_name::<T>()))
            .finish()
    }
}

impl<T: Stateful> Default for StateWidget<T> {
    fn default() -> Self {
        Self { props: None }
    }
}

impl<T: Stateful> Widget for StateWidget<T> {
    type Response = StateResponse<T>;
    type Props<'a> = State<T>;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        let value = self
            .props
            .get_or_insert_with(|| {
                let default = (props.default)();
                Rc::new(RefCell::new(default))
            })
            .clone();

        StateResponse { value }
    }

    fn layout(&self, _: LayoutCtx, _: Constraints) -> Size {
        Size::ZERO
    }
}

pub fn state<T: Stateful>(default: impl FnOnce() -> T + 'static) -> Response<StateResponse<T>> {
    StateWidget::show(State::new(default))
}
