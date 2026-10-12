use crate::data::{Language, TestRecord};

#[derive(Default)]
pub struct HistoryView {
    pub filter: Option<Language>,
    pub scroll: usize,
}

pub fn available_filters(history: &[TestRecord]) -> Vec<Option<Language>> {
    let mut langs = vec![None];
    for r in history {
        let opt = Some(r.language);
        if !langs.contains(&opt) {
            langs.push(opt);
        }
    }
    langs
}
