use crate::{geom::Rect, node::WidgetId};

use slotmap::{Key, SecondaryMap, SlotMap};
use std::fmt::{Debug, Formatter, Result};

pub const fn rect(rect: Rect) -> impl Debug {
    struct Inner(Rect);
    impl Debug for Inner {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "{{({}, {}), ({}, {})}}",
                self.0.min.x, self.0.min.y, self.0.max.x, self.0.max.y
            )
        }
    }
    Inner(rect)
}

pub const fn str(s: &str) -> impl Debug + '_ {
    struct NoQuote<'a>(&'a str);
    impl<'a> Debug for NoQuote<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(self.0)
        }
    }
    NoQuote(s)
}

pub const fn id(id: WidgetId) -> impl Debug {
    struct ShortId(WidgetId);
    impl Debug for ShortId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{:?}", self.0.data())
        }
    }
    ShortId(id)
}

pub const fn vec(list: &Vec<WidgetId>) -> impl Debug + '_ {
    struct Inner<'a>(&'a Vec<WidgetId>);
    impl<'a> Debug for Inner<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_list()
                .entries(self.0.iter().map(|&id| self::id(id)))
                .finish()
        }
    }
    Inner(list)
}

pub const fn slot_map<T: Debug>(map: &SlotMap<WidgetId, T>) -> impl Debug + '_ {
    struct Inner<'a, T>(&'a SlotMap<WidgetId, T>);
    impl<'a, T: Debug> Debug for Inner<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_map()
                .entries(self.0.iter().map(|(k, v)| (self::id(k), v)))
                .finish()
        }
    }
    Inner(map)
}

pub fn secondary_map<T: Debug>(map: &SecondaryMap<WidgetId, T>) -> impl Debug + '_ {
    struct Inner<'a, T>(&'a SecondaryMap<WidgetId, T>);
    impl<'a, T: Debug> Debug for Inner<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_map()
                .entries(self.0.iter().map(|(k, v)| (id(k), v)))
                .finish()
        }
    }
    Inner(map)
}

pub fn short_name(name: &str) -> String {
    const fn is_special(c: char) -> bool {
        matches!(c, ' ' | '<' | '>' | '(' | ')' | '[' | ']' | ',' | ';')
    }

    fn collapse(s: &str) -> &str {
        s.split("::").last().unwrap()
    }

    let mut index = 0;
    let end = name.len();
    let mut out = String::new();

    while index < end {
        let rest = &name[index..end];
        if let Some(mut p) = rest.find(is_special) {
            out.push_str(collapse(&rest[0..p]));

            let ch = &rest[p..=p];
            out.push_str(ch);

            if matches!(ch, ">" | ")" | "]" if rest[p + 1..].starts_with("::")) {
                out.push_str("::");
                p += 2;
            }
            index += p + 1;
        } else {
            out.push_str(collapse(rest));
            index = end;
        }
    }
    out
}
