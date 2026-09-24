//! Code snippet representation and length-tier filtering.
//!
//! Snippet length is measured by newline count rather than words, as code typing
//! difficulty is shaped by vertical block structure and scope depth. Selection
//! automatically falls back across tiers if a language lacks snippets in the requested range.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    #[serde(default)]
    pub id: u32,
    #[serde(alias = "text")]
    pub code: String,
    #[serde(default)]
    pub source: String,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, strum::IntoStaticStr,
)]
pub enum SnippetLength {
    #[default]
    Short,
    Medium,
    Long,
}

impl SnippetLength {
    pub const ALL: [Self; 3] = [Self::Short, Self::Medium, Self::Long];

    pub fn label(self) -> &'static str {
        self.into()
    }

    pub fn matches(self, code: &str) -> bool {
        let lines = code.trim_end().lines().count().max(1);
        match self {
            Self::Short => lines <= 10,
            Self::Medium => (11..=25).contains(&lines),
            Self::Long => lines >= 26,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetData {
    pub language: String,
    pub snippets: Vec<Snippet>,
}

impl SnippetData {
    pub fn random(&self, length: SnippetLength) -> Option<Snippet> {
        let matching: Vec<_> = self
            .snippets
            .iter()
            .filter(|s| length.matches(&s.code))
            .collect();

        if !matching.is_empty() {
            Some(matching[fastrand::usize(..matching.len())].clone())
        } else if !self.snippets.is_empty() {
            Some(self.snippets[fastrand::usize(..self.snippets.len())].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_line_lengths() {
        assert!(SnippetLength::Short.matches(&"line\n".repeat(10)));
        assert!(SnippetLength::Medium.matches(&"line\n".repeat(15)));
        assert!(SnippetLength::Long.matches(&"line\n".repeat(30)));
    }

    #[test]
    fn snippet_data_random_and_fallbacks() {
        let empty_data = SnippetData {
            language: "rust".to_owned(),
            snippets: Vec::new(),
        };
        assert!(empty_data.random(SnippetLength::Short).is_none());

        let short_snippet = Snippet {
            id: 1,
            code: "fn main() {}\n".to_owned(),
            source: "test".to_owned(),
        };
        let data = SnippetData {
            language: "rust".to_owned(),
            snippets: vec![short_snippet.clone()],
        };

        assert_eq!(
            data.random(SnippetLength::Short).unwrap().code,
            short_snippet.code
        );

        assert_eq!(
            data.random(SnippetLength::Long).unwrap().code,
            short_snippet.code
        );
    }
}
