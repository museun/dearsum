use crate::node::WidgetId;

#[derive(Copy, Clone, Debug)]
pub struct Response<R = ()> {
    id: WidgetId,
    inner: R,
}

impl<R> Response<R> {
    pub(crate) const fn new(id: WidgetId, inner: R) -> Self {
        Self { id, inner }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }
}

impl<R> std::ops::Deref for Response<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
