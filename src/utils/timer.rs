use web_time::{Duration, Instant};

/// a stopwatch that can pause. `started` tells a paused timer from one that never ran
#[derive(Debug, Clone, Default)]
pub struct Timer {
    banked: Duration,
    since: Option<Instant>,
    started: bool,
}

impl Timer {
    pub fn start(&mut self) {
        self.started = true;
        self.since.get_or_insert_with(Instant::now);
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn pause(&mut self) {
        if let Some(since) = self.since.take() {
            self.banked += since.elapsed();
        }
    }

    pub fn resume(&mut self) {
        if self.started {
            self.start();
        }
    }

    pub fn seconds(&self) -> f64 {
        let running = self.since.map_or(Duration::ZERO, |since| since.elapsed());
        (self.banked + running).as_secs_f64()
    }

    pub fn is_running(&self) -> bool {
        self.since.is_some()
    }
}
