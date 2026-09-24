//! Supported programming languages and bundled snippet loader.
//!
//! Snippets are inlined at compile time (`include_str!`) so rapid restarts
//! and mid-session language switches never hitch from disk I/O, while keeping
//! the WASM build completely self-contained without runtime network fetches.

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator, IntoStaticStr};

use super::snippet::SnippetData;

#[allow(non_camel_case_types)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Default,
    EnumIter,
    Display,
    IntoStaticStr,
)]
pub enum Language {
    #[default]
    Rust,
    Python,
    JavaScript,
    TypeScript,
    C,
    #[strum(serialize = "C++")]
    Cpp,
    #[strum(serialize = "C#")]
    CSharp,
    Go,
    Java,
    Kotlin,
    PHP,
    Ruby,
    Lua,
    R,
    SQL,
    CSS,
    Dockerfile,
    Bash,
    Arduino,
    Assembly,
    CUDA,
    Jule,
    Julia,
    Nim,
    SystemVerilog,
    VHDL,
    YoptaScript,
}

impl Language {
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    pub fn search(query: &str) -> Vec<Self> {
        let normalized = query.trim().to_lowercase();
        if normalized.is_empty() {
            return Self::all().collect();
        }
        let matcher = SkimMatcherV2::default();
        let mut matches: Vec<_> = Self::all()
            .filter_map(|lang| {
                let label: &'static str = lang.into();
                matcher
                    .fuzzy_match(&label.to_lowercase(), &normalized)
                    .map(|score| (score, lang))
            })
            .collect();
        matches.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
        matches.into_iter().map(|(_, lang)| lang).collect()
    }

    pub fn snippet_data(self) -> SnippetData {
        let json = match self {
            Self::Rust => include_str!("../../assets/snippets/rust.json"),
            Self::Python => include_str!("../../assets/snippets/python.json"),
            Self::JavaScript => include_str!("../../assets/snippets/javascript.json"),
            Self::TypeScript => include_str!("../../assets/snippets/typescript.json"),
            Self::C => include_str!("../../assets/snippets/c.json"),
            Self::Cpp => include_str!("../../assets/snippets/c++.json"),
            Self::CSharp => include_str!("../../assets/snippets/csharp.json"),
            Self::Go => include_str!("../../assets/snippets/go.json"),
            Self::Java => include_str!("../../assets/snippets/java.json"),
            Self::Kotlin => include_str!("../../assets/snippets/kotlin.json"),
            Self::PHP => include_str!("../../assets/snippets/php.json"),
            Self::Ruby => include_str!("../../assets/snippets/ruby.json"),
            Self::Lua => include_str!("../../assets/snippets/lua.json"),
            Self::R => include_str!("../../assets/snippets/r.json"),
            Self::SQL => include_str!("../../assets/snippets/sql.json"),
            Self::CSS => include_str!("../../assets/snippets/css.json"),
            Self::Dockerfile => include_str!("../../assets/snippets/dockerfile.json"),
            Self::Bash => include_str!("../../assets/snippets/bash.json"),
            Self::Arduino => include_str!("../../assets/snippets/arduino.json"),
            Self::Assembly => include_str!("../../assets/snippets/assembly.json"),
            Self::CUDA => include_str!("../../assets/snippets/cuda.json"),
            Self::Jule => include_str!("../../assets/snippets/jule.json"),
            Self::Julia => include_str!("../../assets/snippets/julia.json"),
            Self::Nim => include_str!("../../assets/snippets/nim.json"),
            Self::SystemVerilog => include_str!("../../assets/snippets/systemverilog.json"),
            Self::VHDL => include_str!("../../assets/snippets/vhdl.json"),
            Self::YoptaScript => include_str!("../../assets/snippets/yoptascript.json"),
        };
        serde_json::from_str(json).expect("bundled snippet JSON is valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_loads_its_bundled_data() {
        for language in Language::all() {
            let data = language.snippet_data();
            assert!(
                !data.snippets.is_empty(),
                "Language {language} has no snippets"
            );
            for snippet in &data.snippets {
                assert!(
                    !snippet.code.trim().is_empty(),
                    "Language {language} has empty snippet"
                );
            }
        }
    }

    #[test]
    fn search_languages() {
        let all_count = Language::all().count();
        assert_eq!(Language::search("").len(), all_count);
        assert_eq!(Language::search("rust").first(), Some(&Language::Rust));
        assert_eq!(Language::search("c++").first(), Some(&Language::Cpp));
    }
}
