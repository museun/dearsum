use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub const EMPTY: Self = Self(0);

    pub fn from_ptr<T>(source: &T) -> Self {
        Self::new(source as *const _ as usize)
    }

    pub fn new(source: impl std::hash::Hash) -> Self {
        Self(ahash::RandomState::with_seeds(1, 2, 3, 4).hash_one(source))
    }

    pub fn with(self, source: impl std::hash::Hash) -> Self {
        use std::hash::{BuildHasher as _, Hasher as _};
        let mut hasher = ahash::RandomState::with_seeds(1, 2, 3, 4).build_hasher();
        hasher.write_u64(self.0);
        source.hash(&mut hasher);
        Self(hasher.finish())
    }
}

#[derive(Default)]
pub(crate) struct IdHasher(u64);
impl std::hash::Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }
}

#[derive(Default)]
pub(crate) struct BuildIdHasher;
impl std::hash::BuildHasher for BuildIdHasher {
    type Hasher = IdHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::default()
    }
}

pub(crate) type IdMap<T> = HashMap<Id, T, BuildIdHasher>;
