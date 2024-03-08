use std::{
    collections::{vec_deque, VecDeque},
    ops::RangeBounds,
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct Queue<T, const N: usize> {
    queue: VecDeque<T>,
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        assert!(N > 0, "Queue cannot be empty");
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        while self.queue.len() >= N {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn last(&self) -> Option<&T> {
        self.queue.back()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.queue.front_mut()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.queue.back_mut()
    }

    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> vec_deque::Drain<'_, T> {
        self.queue.drain(range)
    }

    pub fn iter(&self) -> vec_deque::Iter<'_, T> {
        self.queue.iter()
    }

    pub fn iter_mut(&mut self) -> vec_deque::IterMut<'_, T> {
        self.queue.iter_mut()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Queue<T, N> {
    type Item = &'a T;
    type IntoIter = vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Queue<T, N> {
    type Item = &'a mut T;
    type IntoIter = vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const N: usize> IntoIterator for Queue<T, N> {
    type Item = T;
    type IntoIter = vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.queue.into_iter()
    }
}

impl<T, const N: usize> Extend<T> for Queue<T, N> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.queue.extend(iter);
        while self.queue.len() > N {
            self.queue.pop_front();
        }
        self.queue.shrink_to_fit();
    }
}
