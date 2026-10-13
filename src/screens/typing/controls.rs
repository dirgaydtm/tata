use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};
use ratcn::{Theme, list_core::ListItem};

use crate::{
    app::{AppMsg, AppState, OpenDropdown},
    data::{Language, SOUND_OPTIONS, SnippetLength, TestMode},
    ratcn::select::Select,
    screens::Ctx,
};

fn dropdown<T: PartialEq + Clone + 'static>(
    ctx: &mut Ctx<'_>,
    (id, tag): (&'static str, OpenDropdown),
    area: Rect,
    items: impl IntoIterator<Item = ListItem<T>>,
    get: impl Fn(&AppState) -> T + 'static,
    set: impl Fn(T) -> AppMsg + 'static,
) {
    let select = Select::new(items)
        .max_visible_items(40)
        .open(
            move |s: &AppState| s.open_dropdown == Some(tag),
            move |o| AppMsg::OpenDropdown(o.then_some(tag)),
        )
        .selection(move |s: &AppState| Some(get(s)), set);
    ctx.component(id, select, area);
}

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let rows = Layout::horizontal([18, 18, 14, 14].map(Constraint::Length))
        .flex(Flex::Center)
        .spacing(2)
        .areas::<4>(area)
        .map(|column| Layout::vertical([Constraint::Length(1); 2]).areas::<2>(column));

    let label = |text: &'static str| Paragraph::new(text.fg(theme.muted_foreground)).centered();
    for ([label_area, _], text) in rows.iter().zip(["language", "mode", "length", "sound"]) {
        ctx.paint_widget(label(text), *label_area);
    }

    let [[_, lang], [_, mode], [_, len], [_, snd]] = rows;
    let langs = Language::all().map(|l| ListItem::new(l, <&'static str>::from(l)));
    let modes = TestMode::ALL.iter().map(|&m| ListItem::new(m, m.label()));
    let lengths = SnippetLength::ALL
        .iter()
        .map(|&l| ListItem::new(l, l.label()));
    let sounds = SOUND_OPTIONS.iter().map(|&(s, lbl)| ListItem::new(s, lbl));

    dropdown(
        ctx,
        ("lang", OpenDropdown::Language),
        lang,
        langs,
        |s| s.config.language,
        AppMsg::SelectLanguage,
    );
    dropdown(
        ctx,
        ("mode", OpenDropdown::Mode),
        mode,
        modes,
        |s| s.config.test_mode,
        AppMsg::SetMode,
    );
    dropdown(
        ctx,
        ("len", OpenDropdown::Length),
        len,
        lengths,
        |s| s.config.snippet_length,
        AppMsg::SetLength,
    );
    dropdown(
        ctx,
        ("snd", OpenDropdown::Sound),
        snd,
        sounds,
        |s| s.config.sound_enabled,
        AppMsg::SetSound,
    );
}
