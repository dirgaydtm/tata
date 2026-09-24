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
