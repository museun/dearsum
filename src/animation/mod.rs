use std::time::Duration;

pub mod easing;

mod id;
use self::id::IdMap;
pub use id::Id;

use crate::geom::math::remap_clamp;

#[derive(Default)]
pub struct Manager {
    bools: IdMap<Bool>,
    values: IdMap<Value>,
    steady: Duration,
}

impl Manager {
    pub fn clear(&mut self) {
        self.bools.clear();
        self.values.clear();
    }

    // remove_bool
    // remove_value

    // TODO animation time is interpolated from 0.0..=1.0
    pub fn animate_bool(&mut self, id: Id, dt: Duration, value: bool, animation_time: f32) -> f32 {
        use std::collections::hash_map::Entry;
        let (start, end) = if value { (0.0, 1.0) } else { (1.0, 0.0) };

        let current = self.steady.as_secs_f32();
        match self.bools.entry(id) {
            Entry::Occupied(mut entry) => {
                let val = entry.get_mut();

                let elapsed = (current - val.time).clamp(0.0, current);
                let new = val.last + (end - start) * elapsed / animation_time;

                val.last = if new.is_finite() {
                    new.clamp(0.0, 1.0)
                } else {
                    end
                };
                val.time = current;

                val.last
            }

            Entry::Vacant(entry) => {
                entry.insert(Bool {
                    last: end,
                    time: current - dt.as_secs_f32(),
                });
                end
            }
        }
    }

    pub fn animate_value(&mut self, id: Id, value: f32, animation_time: f32) -> f32 {
        use std::collections::hash_map::Entry;

        let current_time = self.steady.as_secs_f32();
        match self.values.entry(id) {
            Entry::Occupied(mut entry) => {
                let val = entry.get_mut();
                let elapsed = current_time - val.toggle;
                let current = remap_clamp(elapsed, (0.0, animation_time), (val.from, val.to));
                if val.to != value {
                    val.from = current;
                    val.to = value;
                    val.toggle = current_time
                }
                if animation_time == 0.0 {
                    val.from = value;
                    val.to = value;
                }
                current
            }
            Entry::Vacant(entry) => {
                entry.insert(Value {
                    from: value,
                    to: value,
                    toggle: -f32::INFINITY,
                });
                value
            }
        }
    }

    pub(crate) fn tick(&mut self, steady: Duration) {
        self.steady = steady;
    }
}

#[derive(Copy, Clone, Debug)]
struct Bool {
    last: f32,
    time: f32,
}

struct Value {
    from: f32,
    to: f32,
    toggle: f32,
}
