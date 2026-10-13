use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};

use crate::{
    app::{AppMsg, AppState},
    data::{CaretStyle, SOUND_OPTIONS, SnippetLength, TestMode, ThemeChoice},
    ratcn::tabs::{Tab, Tabs},
    screens::Ctx,
};

fn render_setting_tabs<T: PartialEq + Clone + 'static>(
    ctx: &mut Ctx<'_>,
    tab: usize,
    title: &str,
    items: impl IntoIterator<Item = Tab<T>>,
    get: impl Fn(&AppState) -> T + 'static,
    set: impl Fn(T) -> AppMsg + 'static,
    area: Rect,
) {
    let active = ctx.state().settings_view.tab == tab;
    let theme = ctx.state().theme();
    let [title_area, tabs_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
    let color = if active {
        theme.primary
    } else {
        theme.secondary
    };
    let prefix = if active { "▶ " } else { "  " };

    ctx.paint_widget(
        Paragraph::new(format!("{prefix}[{}] {title}", tab + 1).fg(color).bold()),
        title_area,
    );
    ctx.component(
        format!("opt_tab_{tab}"),
        Tabs::new(items).selection(move |s: &AppState| Some(get(s)), set),
        tabs_area,
    );
}

pub fn draw(ctx: &mut Ctx<'_>, area: Rect) {
    let [mode, length, caret, sound, theme] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .spacing(1)
    .areas(area);

    render_setting_tabs(
        ctx,
        1,
        "Test Mode",
        TestMode::ALL.iter().map(|&m| Tab::new(m, m.label())),
        |s| s.config.test_mode,
        AppMsg::SetMode,
        mode,
    );
    render_setting_tabs(
        ctx,
        2,
        "Snippet Length",
        SnippetLength::ALL.iter().map(|&l| Tab::new(l, l.label())),
        |s| s.config.snippet_length,
        AppMsg::SetLength,
        length,
    );
    render_setting_tabs(
        ctx,
        3,
        "Caret Style",
        CaretStyle::ALL.iter().map(|&c| Tab::new(c, c.label())),
        |s| s.config.caret_style,
        AppMsg::SetCaret,
        caret,
    );
    render_setting_tabs(
        ctx,
        4,
        "Sound Feedback",
        SOUND_OPTIONS.iter().map(|&(snd, lbl)| Tab::new(snd, lbl)),
        |s| s.config.sound_enabled,
        AppMsg::SetSound,
        sound,
    );
    render_setting_tabs(
        ctx,
        5,
        "Theme",
        ThemeChoice::ALL.iter().map(|&t| Tab::new(t, t.label())),
        |s| s.config.theme,
        AppMsg::SetTheme,
        theme,
    );
}
