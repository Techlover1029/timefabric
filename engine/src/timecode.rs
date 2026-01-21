#[derive(Debug, Clone, Copy)]
pub struct Timecode {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
}

impl Timecode {
    pub fn new() -> Self {
        Self { hours: 0, minutes: 0, seconds: 0, frames: 0 }
    }

    pub fn tick(&mut self, frame_rate: crate::FrameRate) {
        self.frames += 1;
        let fps = match frame_rate {
            crate::FrameRate::Fps24 => 24,
            crate::FrameRate::Fps25 => 25,
            crate::FrameRate::Fps30 => 30,
            crate::FrameRate::Fps30Drop => 30, // drop-frame counting not implemented yet
            crate::FrameRate::Fps50 => 50,
            crate::FrameRate::Fps60 => 60,
            crate::FrameRate::Fps60Drop => 60, // drop-frame counting not implemented yet
        };

        if self.frames >= fps {
            self.frames = 0;
            self.seconds += 1;
        }
        if self.seconds >= 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
        if self.minutes >= 60 {
            self.minutes = 0;
            self.hours += 1;
        }
    }
}