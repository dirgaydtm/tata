use super::stats::TypingStats;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionStatus {
    Idle,
    Active,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharStatus {
    #[default]
    Untyped,
    Correct,
    Incorrect(char),
}

#[derive(Debug, Clone)]
pub struct Session {
    chars: Vec<char>,
    statuses: Vec<CharStatus>,
    index: usize,
    line_start: usize,
    status: SessionStatus,
    pub stats: TypingStats,
}

/// cleans the snippet so nothing invisible (tabs, trailing spaces, blank lines at the
/// ends) needs typing
fn normalize_code(raw: &str) -> Vec<char> {
    raw.replace('\t', "    ")
        .replace('\r', "")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_start_matches('\n')
        .trim_end()
        .chars()
        .collect()
}

impl Session {
    pub fn new(code: &str) -> Self {
        let chars = normalize_code(code);
        let mut session = Self {
            statuses: vec![CharStatus::Untyped; chars.len()],
            chars,
            index: 0,
            line_start: 0,
            status: SessionStatus::Idle,
            stats: TypingStats::default(),
        };
        session.skip_indent();
        // an empty snippet is already done before it starts
        session.finish_if_complete();
        session
    }

    pub fn code(&self) -> &[char] {
        &self.chars
    }

    pub fn statuses(&self) -> &[CharStatus] {
        &self.statuses
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn progress(&self) -> f64 {
        self.index as f64 / self.chars.len().max(1) as f64
    }

    pub fn is_complete(&self) -> bool {
        self.status == SessionStatus::Completed
    }

    fn expected(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    /// `'\n'` is how Enter gets in here: it's only accepted on a newline, and a newline
    /// only accepts Enter
    pub fn type_char(&mut self, typed: char) {
        let expected = self.expected();
        if self.is_complete() || (expected == Some('\n')) != (typed == '\n') {
            return;
        }
        self.commit(if expected == Some(typed) {
            CharStatus::Correct
        } else {
            CharStatus::Incorrect(typed)
        });
        if typed == '\n' {
            self.skip_indent();
        }
    }

    /// ends the test early (time ran out). that's why "complete" is stored and not worked
    /// out from the cursor, `go_to` relies on it
    pub fn complete(&mut self) {
        self.status = SessionStatus::Completed;
    }

    pub fn has_started(&self) -> bool {
        self.status != SessionStatus::Idle
    }

    fn commit(&mut self, status: CharStatus) {
        self.status = SessionStatus::Active;
        self.stats.record(status == CharStatus::Correct);
        self.statuses[self.index] = status;
        self.index += 1;
        self.finish_if_complete();
    }

    /// indent counts as done but not as typed, so it never touches the stats
    fn skip_indent(&mut self) {
        while self.chars.get(self.index) == Some(&' ') {
            self.statuses[self.index] = CharStatus::Correct;
            self.index += 1;
        }
        self.line_start = self.index;
    }

    fn finish_if_complete(&mut self) {
        if self.index >= self.chars.len() {
            self.complete();
        }
    }
}
