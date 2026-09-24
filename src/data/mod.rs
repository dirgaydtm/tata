pub mod config;
pub mod languages;
pub mod snippet;
pub mod storage;

pub use config::{CaretStyle, SOUND_OPTIONS, TestMode, ThemeChoice, UserConfig};
pub use languages::Language;
pub use snippet::{Snippet, SnippetData, SnippetLength};
pub use storage::{AppError, TestRecord, load_config, load_history, save_config, save_history};
