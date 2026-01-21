use crate::{Clock, ClockId, FrameRate, Timecode};
use std::collections::BTreeMap;

pub struct ClockManager {
    clocks: BTreeMap<ClockId, Clock>,
    next_id: u32,
}

impl ClockManager {
    pub fn new() -> Self {
        Self {
            clocks: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn create_clock(&mut self, frame_rate: FrameRate) -> ClockId {
        let id = ClockId(self.next_id);
        self.next_id += 1;
        let clock = Clock {
            timecode: Timecode::new(),
            running: true,
            frame_rate,
        };
        self.clocks.insert(id, clock);
        id
    }

    pub fn tick_all(&mut self) {
        for clock in self.clocks.values_mut() {
            clock.tick();
        }
    }

    pub fn print_all(&self) {
        use std::io::{stdout, Write};
        stdout().flush().unwrap();
        for (id, clock) in &self.clocks {
            let t = &clock.timecode;
            print!(
                "\rClock {:?} -> {:02}:{:02}:{:02}:{:02}",
                id, t.hours, t.minutes, t.seconds, t.frames
            );
        }
        stdout().flush().unwrap();
    }

    pub fn get_clock_mut(&mut self, id: ClockId) -> Option<&mut Clock> {
        self.clocks.get_mut(&id)
    }
}