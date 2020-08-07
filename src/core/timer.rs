use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
}

impl Timer {
    /// Creates a new timer
    pub fn now() -> Self {
        Timer {
            start_time: Instant::now(),
        }
    }

    /// Starts the timer.
    /// Basically it sets the start_time to the current instant.
    ///
    /// It returns the new start time
    ///
    pub fn start(&mut self) -> &Instant {
        self.start_time = Instant::now();
        &self.start_time
    }

    /// Returns the elapsed time since start as Duration
    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }

    /// Returns the elapsed time since start as seconds
    pub fn elapsed_in_seconds(&self) -> f64 {
        let elapsed = self.elapsed();
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
    }
}
