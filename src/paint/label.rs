use std::{borrow::Cow, cell::Ref, rc::Rc};

use crate::geom::{vec2, Vec2};

pub trait Label: std::fmt::Debug {
    type Static: Label + 'static;
    fn into_static(self) -> Self::Static;
    fn size(&self) -> Vec2;
    fn chars(&self) -> impl Iterator<Item = char>;
}

impl Label for () {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        std::iter::empty()
    }
}

impl<T: Label + 'static> Label for Rc<T> {
    type Static = Rc<T>;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        <T as Label>::size(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        <T as Label>::chars(self)
    }
}

impl<'a> Label for Ref<'a, str> {
    type Static = String;

    fn into_static(self) -> Self::Static {
        self.to_string()
    }

    fn size(&self) -> Vec2 {
        size_of_str(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        (**self).chars()
    }
}

impl<T: Label> Label for &T
where
    T: Clone,
{
    type Static = T::Static;

    fn into_static(self) -> Self::Static {
        <T as Label>::into_static(self.clone())
    }

    fn size(&self) -> Vec2 {
        <T as Label>::size(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        <T as Label>::chars(*self)
    }
}

impl Label for bool {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        vec2(if *self { 4 } else { 5 }, 1)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        match *self {
            true => "true",
            false => "false",
        }
        .chars()
    }
}

impl Label for char {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        vec2(1, 1)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        std::iter::once(*self)
    }
}

impl Label for &'static str {
    type Static = &'static str;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        size_of_str(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        str::chars(self)
    }
}

impl Label for String {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        size_of_str(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        str::chars(self)
    }
}

impl<'a> Label for Cow<'a, str> {
    type Static = Cow<'static, str>;

    fn into_static(self) -> Self::Static {
        match self {
            Cow::Borrowed(s) => Cow::Owned(s.to_owned()),
            Cow::Owned(s) => Cow::Owned(s),
        }
    }

    fn size(&self) -> Vec2 {
        size_of_str(self)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        str::chars(self)
    }
}

impl Label for i32 {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        vec2(count_signed_digits(*self as isize) as i32, 1)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        signed_digits(*self as isize)
    }
}
impl Label for usize {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        vec2(count_digits(*self) as i32, 1)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        digits(*self).map(int_to_char)
    }
}

impl Label for f32 {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }

    fn size(&self) -> Vec2 {
        vec2(count_float_digits(*self) as i32, 1)
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        let (head, tail) = float_digits(*self);
        head.map(int_to_char)
            .chain(Some('.'))
            .chain(tail.map(int_to_char))
    }
}

const fn int_to_char(c: u8) -> char {
    (c + b'0') as char
}

const fn count_digits(d: usize) -> usize {
    let (mut len, mut n) = (1, 1);
    while len < 20 {
        n *= 10;
        if n > d {
            return len;
        }
        len += 1;
    }
    len
}

fn digits(mut d: usize) -> impl Iterator<Item = u8> {
    let x = count_digits(d) as u32 - 1;
    let mut mag = 10usize.pow(x);
    if d < mag {
        mag /= 10;
    }
    let mut is_zero = d == 0;
    std::iter::from_fn(move || {
        if std::mem::take(&mut is_zero) {
            return Some(0);
        }
        if mag == 0 {
            return None;
        }
        let n = d / mag;
        d %= mag;
        mag /= 10;
        Some(n as u8)
    })
}

fn count_signed_digits(d: isize) -> usize {
    let signed = d.is_negative() as usize;
    let len = count_digits(d.unsigned_abs());
    len + signed
}

fn signed_digits(d: isize) -> impl Iterator<Item = char> {
    d.is_negative()
        .then_some('-')
        .into_iter()
        .chain(digits(d.unsigned_abs()).map(int_to_char))
}

fn count_float_digits(f: f32) -> usize {
    let (head, tail) = split_f32(f);
    count_digits(head) + 2 + count_leading(tail)
}

fn float_digits(f: f32) -> (impl Iterator<Item = u8>, impl Iterator<Item = u8>) {
    let (head, tail) = split_f32(f);
    (digits(head), digits(tail).take(count_leading(tail) + 1))
}

fn count_leading(f: usize) -> usize {
    let mut p = 0;
    digits(f)
        .enumerate()
        .take_while(|&(i, c)| {
            p += (i > 0 && (c == 0)) as usize;
            p < 1
        })
        .map(|(s, _)| s)
        .last()
        .unwrap_or(0)
}

fn split_f32(f: f32) -> (usize, usize) {
    let head = f.trunc() as usize;
    let tail = (f.fract() * 1e2) as usize;
    (head, tail)
}

fn size_of_str(s: &str) -> Vec2 {
    let mut size = vec2(0, 1);
    let mut max_x = 0;
    for ch in s.chars() {
        if ch == '\n' {
            size.y += 1;
            size.x = std::mem::take(&mut max_x).max(size.x);
            continue;
        }
        max_x += 1;
    }
    size.x = size.x.max(max_x);
    size
}
