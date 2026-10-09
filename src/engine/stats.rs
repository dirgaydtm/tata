const SECONDS_PER_MINUTE: f64 = 60.0;

#[derive(Debug, Clone, Default)]
pub struct TypingStats {
    total_keystrokes: usize,
    correct_keystrokes: usize,
    correct_chars: usize,
    uncorrected_errors: usize,
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
}

fn rate(chars: usize, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        0.0
    } else {
        chars as f64 / (elapsed_seconds / SECONDS_PER_MINUTE)
    }
}
