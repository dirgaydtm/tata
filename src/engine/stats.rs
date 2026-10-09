//! Typing metrics: raw and net CPM, accuracy, and consistency.
//!
//! Speed is measured in characters per minute because a "word" in code is a fuzzy
//! idea. Net CPM only counts characters that are correct right now, so erasing and
//! retyping never inflates the score, while raw CPM and accuracy keep counting every
//! keystroke that was pressed.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_standard_typing_metrics() {
        let mut stats = TypingStats::default();
        for _ in 0..25 {
            stats.record(true);
        }
        assert_eq!(stats.raw_cpm(60.0), 25.0);
        assert_eq!(stats.net_cpm(60.0), 25.0);
        assert_eq!(stats.accuracy(), 100.0);
    }

    #[test]
    fn samples_measure_each_interval_not_a_running_average() {
        let mut stats = TypingStats::default();
        for _ in 0..10 {
            stats.record(true);
        }
        stats.tick(0.5);
        stats.tick(1.0);
        for _ in 0..5 {
            stats.record(true);
        }
        stats.tick(1.5);
        stats.tick(2.0);
        assert_eq!(stats.samples(), &[600.0, 300.0]);
    }

    #[test]
    fn net_cpm_counts_only_characters_correct_right_now() {
        let mut stats = TypingStats::default();
        for _ in 0..10 {
            stats.record(true);
        }
        stats.record(false);
        assert_eq!(stats.raw_cpm(60.0), 11.0);
        assert_eq!(stats.net_cpm(60.0), 10.0);

        stats.erase(true);
        assert_eq!(stats.net_cpm(60.0), 9.0);
        assert_eq!(stats.raw_cpm(60.0), 11.0);
    }

    #[test]
    fn consistency_drops_with_varied_samples() {
        let mut stats = TypingStats::default();
        stats.record(true);
        stats.tick(1.0);
        for _ in 0..3 {
            stats.record(true);
        }
        stats.tick(2.0);
        // samples are 60 and 180 CPM: mean 120, sigma 60, so 100 - 50%
        assert!((stats.consistency() - 50.0).abs() < 1e-9);
    }

    #[test]
    fn non_positive_elapsed_time_yields_zero_cpm() {
        let mut stats = TypingStats::default();
        stats.record(true);
        assert_eq!(stats.raw_cpm(0.0), 0.0);
        assert_eq!(stats.net_cpm(0.0), 0.0);
    }
}
