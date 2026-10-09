const SECONDS_PER_MINUTE: f64 = 60.0;

#[derive(Debug, Clone, Default)]
pub struct TypingStats {
    total_keystrokes: usize,
    correct_keystrokes: usize,
    correct_chars: usize,
    uncorrected_errors: usize,
    last_sample_time: f64,
    last_sample_keystrokes: usize,
    samples: Vec<f64>,
}

impl TypingStats {
    /// counts one keystroke. totals only go up, but `correct_chars` and
    /// `uncorrected_errors` shrink again when you backspace via `erase`
    pub(super) fn record(&mut self, correct: bool) {
        self.total_keystrokes += 1;
        if correct {
            self.correct_keystrokes += 1;
            self.correct_chars += 1;
        } else {
            self.uncorrected_errors += 1;
        }
    }

    pub(super) fn erase(&mut self, was_correct: bool) {
        if was_correct {
            self.correct_chars -= 1;
        } else {
            self.uncorrected_errors -= 1;
        }
    }

    pub const fn uncorrected_errors(&self) -> usize {
        self.uncorrected_errors
    }

    /// samples the speed since the last sample (at most once a second), not a running
    /// average. a running average just flattens out and makes consistency pointless
    pub fn tick(&mut self, elapsed_seconds: f64) {
        let interval = elapsed_seconds - self.last_sample_time;
        if interval >= 1.0 {
            let typed = self.total_keystrokes - self.last_sample_keystrokes;
            self.samples.push(rate(typed, interval));
            self.last_sample_time = elapsed_seconds;
            self.last_sample_keystrokes = self.total_keystrokes;
        }
    }

    pub fn samples(&self) -> &[f64] {
        &self.samples
    }

    pub fn raw_cpm(&self, elapsed_seconds: f64) -> f64 {
        rate(self.total_keystrokes, elapsed_seconds)
    }

    pub fn net_cpm(&self, elapsed_seconds: f64) -> f64 {
        rate(self.correct_chars, elapsed_seconds)
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_keystrokes == 0 {
            100.0
        } else {
            self.correct_keystrokes as f64 / self.total_keystrokes as f64 * 100.0
        }
    }

    /// 100 minus the coefficient of variation (%), so it doesn't care how fast you type
    pub fn consistency(&self) -> f64 {
        let count = self.samples.len() as f64;
        let mean = self.samples.iter().sum::<f64>() / count;
        if self.samples.is_empty() || mean <= 0.0 {
            return 100.0;
        }
        let variance = self
            .samples
            .iter()
            .map(|sample| (sample - mean).powi(2))
            .sum::<f64>()
            / count;
        (100.0 - variance.sqrt() / mean * 100.0).max(0.0)
    }
}

fn rate(chars: usize, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        0.0
    } else {
        chars as f64 / (elapsed_seconds / SECONDS_PER_MINUTE)
    }
}
