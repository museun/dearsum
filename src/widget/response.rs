use crate::{node::WidgetId, NoResponse};

pub type UserResponse<R> = Response<NoResponse, R>;

#[derive(Copy, Clone, Debug)]
pub struct Response<R = (), T = ()> {
    pub(crate) id: WidgetId,
    pub(crate) inner: R,
    pub(crate) output: T,
}

impl<R, T> Response<R, T> {
    pub(crate) const fn new(id: WidgetId, inner: R, output: T) -> Self {
        Self { id, inner, output }
    }

    pub fn take(self) -> R {
        self.inner
    }

    pub fn into_output(self) -> T {
        self.output
    }

    pub fn output(&self) -> &T {
        &self.output
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }
}

impl<R> Response<R, ()> {
    pub(crate) fn map<T>(self, output: T) -> Response<R, T> {
        Response {
            id: self.id,
            inner: self.inner,
            output,
        }
    }
}

impl<R, T> std::ops::Deref for Response<R, T> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
