//! User configuration and runtime test preferences.
//!
//! Selection enums derive `Copy` so preference state flows freely through UI
//! component trees and event loops without heap allocation or clone overhead.

use serde::{Deserialize, Serialize};

use super::{Language, SnippetLength};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, strum::IntoStaticStr,
)]
pub enum CaretStyle {
    #[default]
    #[strum(serialize = "Block (█)")]
    Block,
    #[strum(serialize = "Line (│)")]
    Line,
    #[strum(serialize = "Underline (_)")]
    Underline,
}

impl CaretStyle {
    pub const ALL: [Self; 3] = [Self::Block, Self::Line, Self::Underline];

    pub fn label(self) -> &'static str {
        self.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TestMode {
    #[default]
    FullSnippet,
    Countdown(u16),
}

impl TestMode {
    pub const ALL: [Self; 4] = [
        Self::FullSnippet,
        Self::Countdown(15),
        Self::Countdown(30),
        Self::Countdown(60),
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSnippet => "Full Snippet",
            Self::Countdown(15) => "15s",
            Self::Countdown(30) => "30s",
            Self::Countdown(60) => "60s",
            Self::Countdown(_) => "Custom",
        }
    }

    pub const fn duration(self) -> Option<u16> {
        match self {
            Self::FullSnippet => None,
            Self::Countdown(seconds) => Some(seconds),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, strum::IntoStaticStr,
)]
pub enum ThemeChoice {
    #[default]
    Catppuccin,
    Nord,
    #[strum(serialize = "Tokyo Night")]
    TokyoNight,
    Gruvbox,
    Terminal,
    Dark,
}

impl ThemeChoice {
    pub const ALL: [Self; 6] = [
        Self::Catppuccin,
        Self::Nord,
        Self::TokyoNight,
        Self::Gruvbox,
        Self::Terminal,
        Self::Dark,
    ];

    pub fn label(self) -> &'static str {
        self.into()
    }
}

pub const SOUND_OPTIONS: [(bool, &str); 2] = [(true, "Sound On"), (false, "Sound Off")];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserConfig {
    pub language: Language,
    pub snippet_length: SnippetLength,
    pub test_mode: TestMode,
    pub caret_style: CaretStyle,
    pub theme: ThemeChoice,
    pub sound_enabled: bool,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            language: Language::Rust,
            snippet_length: SnippetLength::Short,
            test_mode: TestMode::FullSnippet,
            caret_style: CaretStyle::Block,
            theme: ThemeChoice::Catppuccin,
            sound_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_config_roundtrips_serialization() {
        let config = UserConfig::default();
        let json = serde_json::to_string(&config).expect("Serialize config");
        let parsed: UserConfig = serde_json::from_str(&json).expect("Deserialize config");
        assert_eq!(config, parsed);
    }
}
