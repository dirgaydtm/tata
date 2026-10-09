//! Typing session state machine for a single snippet.
//!
//! Backspace never crosses a committed line, which keeps corrections local and the
//! line you just finished safely locked in.

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

    pub fn backspace(&mut self) {
        if self.index <= self.line_start || self.is_complete() {
            return;
        }
        self.index -= 1;
        self.stats
            .erase(self.statuses[self.index] == CharStatus::Correct);
        self.statuses[self.index] = CharStatus::Untyped;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entering_a_line_skips_the_next_line_indent() {
        let mut session = Session::new("a\n  b");
        session.type_char('a');
        session.type_char('\n');
        assert_eq!(session.expected(), Some('b'));
        assert_eq!(session.index(), 4);
        assert_eq!(
            &session.statuses()[2..4],
            &[CharStatus::Correct, CharStatus::Correct]
        );
        // only the typed `a` and the Enter count, not the skipped indent
        assert_eq!(session.stats.net_cpm(60.0), 2.0);
    }

    #[test]
    fn backspace_cannot_cross_a_committed_line() {
        let mut session = Session::new("a\nb");
        session.type_char('a');
        session.type_char('\n');
        let index = session.index();
        session.backspace();
        assert_eq!(session.index(), index);
    }

    #[test]
    fn normalization_removes_invisible_keystrokes() {
        let session = Session::new("\n\n\tif x {\r\n  y  \r\n}\n\n");
        let code: String = session.code().iter().collect();
        assert_eq!(code, "    if x {\n  y\n}");
        assert_eq!(session.index(), 4);
    }

    #[test]
    fn retyping_erased_characters_does_not_inflate_net_cpm() {
        let mut session = Session::new("abc");
        session.type_char('a');
        session.type_char('x');
        session.backspace();
        session.backspace();
        for c in ['a', 'b', 'c'] {
            session.type_char(c);
        }
        assert!(session.is_complete());
        assert_eq!(session.stats.net_cpm(60.0), 3.0);
        assert_eq!(session.stats.raw_cpm(60.0), 5.0);
    }

    #[test]
    fn empty_code_is_complete_and_rejects_input() {
        let mut session = Session::new("");
        assert!(session.is_complete());
        session.type_char('a');
        session.type_char('\n');
        session.backspace();
        assert_eq!(session.stats.raw_cpm(60.0), 0.0);
    }

    #[test]
    fn wrong_char_is_recorded_and_backspace_clears_it() {
        let mut session = Session::new("ab");
        session.type_char('x');
        assert_eq!(session.statuses()[0], CharStatus::Incorrect('x'));
        assert_eq!(session.stats.uncorrected_errors(), 1);
        session.backspace();
        assert_eq!(session.statuses()[0], CharStatus::Untyped);
        assert_eq!(session.stats.uncorrected_errors(), 0);
    }

    #[test]
    fn typing_every_char_completes_the_session() {
        let mut session = Session::new("a\nb");
        assert!(!session.has_started());
        assert_eq!(session.progress(), 0.0);
        session.type_char('a');
        assert!(session.has_started());
        assert!(!session.is_complete());
        session.type_char('\n');
        session.type_char('b');
        assert!(session.is_complete());
        assert_eq!(session.progress(), 1.0);
    }
}
