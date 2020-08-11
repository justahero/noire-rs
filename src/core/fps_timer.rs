use super::Timer;
use std::collections::VecDeque;

static MAX_FPS_COUNT: usize = 50;

pub struct FpsTimer {
    /// The timer to get elapsed time for
    timer: Timer,
    /// The list of time stamps, ideally on a frame base
    frames: VecDeque<f64>,
    /// The last time stamp
    last_time: f64,
    /// Total frames measured since start
    total_frames: usize,
}

impl FpsTimer {
    /// Create a new Fps Timer
    pub fn now() -> Self {
        Self {
            timer: Timer::now(),
            frames: VecDeque::new(),
            last_time: 0.0,
            total_frames: 0,
        }
    }

    /// Take an elapsed time stamp since previous frame and store it
    pub fn next_frame(&mut self) -> f64 {
        // get diff between current time and last time
        let current_time = self.timer.elapsed_in_seconds();
        let elapsed = current_time - self.last_time;
        self.last_time = current_time;

        self.frames.push_back(elapsed);
        if self.frames.len() >= MAX_FPS_COUNT {
            self.frames.pop_front();
        }

        self.total_frames += 1;

        elapsed
    }

    /// Calculate the average frame rate of the last N frames
    pub fn fps(&self) -> f64 {
        let fps: f64 = self.frames.iter().sum();
        1.0 / fps * (self.frames.len() as f64)
    }

    /// Returns the total number frames measured
    pub fn total_frames(&self) -> usize {
        self.total_frames
    }
}
