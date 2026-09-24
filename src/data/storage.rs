use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use super::{Language, UserConfig};

const CONFIG_FILE: &str = "config.json";
const HISTORY_FILE: &str = "history.json";
const CONFIG_KEY: &str = "tata-config";
const HISTORY_KEY: &str = "tata-history";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRecord {
    pub id: String,
    pub timestamp: i64,
    pub language: Language,
    pub raw_wpm: f64,
    pub net_wpm: f64,
    pub accuracy: f64,
    pub duration_seconds: f64,
    pub total_chars: usize,
    pub error_chars: usize,
}

#[derive(Debug)]
pub enum AppError {
    Json(serde_json::Error),
    #[cfg(not(target_arch = "wasm32"))]
    Io(std::io::Error),
    Storage(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => e.fmt(f),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Io(e) => e.fmt(f),
            Self::Storage(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

pub fn load_config() -> Result<UserConfig, AppError> {
    match load(CONFIG_FILE, CONFIG_KEY)? {
        Some(raw) => Ok(serde_json::from_str(&raw)?),
        None => Ok(UserConfig::default()),
    }
}

pub fn save_config(config: &UserConfig) -> Result<(), AppError> {
    save(CONFIG_FILE, CONFIG_KEY, &serde_json::to_string(config)?)
}

pub fn load_history() -> Result<Vec<TestRecord>, AppError> {
    match load(HISTORY_FILE, HISTORY_KEY)? {
        Some(raw) => Ok(serde_json::from_str(&raw)?),
        None => Ok(Vec::new()),
    }
}

pub fn save_history(history: &[TestRecord]) -> Result<(), AppError> {
    save(HISTORY_FILE, HISTORY_KEY, &serde_json::to_string(history)?)
}

#[cfg(not(target_arch = "wasm32"))]
fn data_path(file: &str) -> Result<PathBuf, AppError> {
    let dir = dirs::config_dir()
        .ok_or_else(|| AppError::Storage("could not determine config dir".into()))?
        .join("tata");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(file))
}

#[cfg(not(target_arch = "wasm32"))]
fn load(file: &str, _key: &str) -> Result<Option<String>, AppError> {
    match std::fs::read_to_string(data_path(file)?) {
        Ok(raw) => Ok(Some(raw)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save(file: &str, _key: &str, raw: &str) -> Result<(), AppError> {
    std::fs::write(data_path(file)?, raw)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn storage() -> Result<web_sys::Storage, AppError> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .ok_or_else(|| AppError::Storage("localStorage is unavailable".into()))
}

#[cfg(target_arch = "wasm32")]
fn load(_file: &str, key: &str) -> Result<Option<String>, AppError> {
    storage()?
        .get_item(key)
        .map_err(|_| AppError::Storage("could not read localStorage".into()))
}

#[cfg(target_arch = "wasm32")]
fn save(_file: &str, key: &str, raw: &str) -> Result<(), AppError> {
    storage()?
        .set_item(key, raw)
        .map_err(|_| AppError::Storage("could not write localStorage".into()))
}
