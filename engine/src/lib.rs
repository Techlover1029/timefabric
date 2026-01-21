mod timecode;
pub use timecode::Timecode;

mod clock_manager;
pub use clock_manager::ClockManager; // <-- this line exposes ClockManager

pub mod node;

pub mod graph;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub struct ClockId(pub u32);
use std::sync::atomic::{AtomicU32, Ordering};

static CLOCK_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

impl ClockId {
    pub fn new() -> Self {
        ClockId(CLOCK_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

// FrameRate enum - supports major professional frame rates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    Fps24,
    Fps25,
    Fps30,
    Fps30Drop, // 29.97 drop-frame
    Fps50,
    Fps60,
    Fps60Drop, // 59.94 drop-frame
}

impl FrameRate {
    pub fn as_u8(&self) -> u8 {
        match self {
            FrameRate::Fps24 => 24,
            FrameRate::Fps25 => 25,
            FrameRate::Fps30 => 30,
            FrameRate::Fps30Drop => 29,
            FrameRate::Fps50 => 50,
            FrameRate::Fps60 => 60,
            FrameRate::Fps60Drop => 59,
        }
    }
}

// TimecodeFrame struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimecodeFrame {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
}

impl TimecodeFrame {
    pub fn new(hours: u8, minutes: u8, seconds: u8, frames: u8) -> Self {
        Self { hours, minutes, seconds, frames }
    }

    // Convert to total frames (non-drop-frame)
    pub fn total_frames(&self, rate: FrameRate) -> u64 {
        let fps = rate.as_u8();
        ((self.hours as u64 * 3600 + self.minutes as u64 * 60 + self.seconds as u64) as f64 * fps as f64
            + self.frames as f64)
            .round() as u64
    }
}

// Clock states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockState {
    Stopped,
    Running,
    Paused,
    Chasing,
}

#[derive(Debug)]
pub struct Clock {
    pub timecode: Timecode,
    pub running: bool,
    pub frame_rate: FrameRate,
}

impl Clock {
    pub fn tick(&mut self) {
        if self.running {
            self.timecode.tick(self.frame_rate);
        }
    }

    pub fn play(&mut self) { self.running = true; }
    pub fn pause(&mut self) { self.running = false; }
    pub fn reset(&mut self) { self.timecode = Timecode::new(); }
}
