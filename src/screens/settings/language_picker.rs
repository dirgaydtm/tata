use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};
use ratcn::Theme;

use crate::{app::AppMsg, data::Language, ratcn::button::Button, screens::Ctx};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let state = ctx.state();
    let active = state.settings_view.tab == 0;
    let [search_box, list_box] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
        .spacing(1)
        .areas(area);

    let mut spans = vec![" 🔍 ".fg(theme.primary)];
    if state.settings_view.query.is_empty() {
        spans.push("Type to filter...".fg(theme.muted_foreground));
    } else {
        spans.push(
            state
                .settings_view
                .query
                .clone()
                .fg(theme.foreground)
                .bold(),
        );
        spans.push("█".fg(theme.secondary));
    }
    let search_content = Line::from(spans);

    let (border, title, title_color) = if active {
        (theme.primary, " [1] Languages (Active) ", theme.primary)
    } else {
        (theme.border, " [1] Languages ", theme.muted_foreground)
    };
    ctx.paint_widget(
        Paragraph::new(search_content).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(border))
                .title(title)
                .title_style(Style::new().fg(title_color).bold()),
        ),
        search_box,
    );

    let languages = Language::search(&state.settings_view.query);
    if languages.is_empty() {
        ctx.paint_widget(
            Paragraph::new("  No matching languages.")
                .style(Style::new().fg(theme.muted_foreground)),
            list_box,
        );
        return;
    }

    let visible_rows = (list_box.height as usize).max(1);
    let selection = state.settings_view.selection;
    let scroll = selection.saturating_sub(visible_rows.saturating_sub(1));
    for (row, &lang) in languages.iter().skip(scroll).take(visible_rows).enumerate() {
        let idx = scroll + row;
        let selected = idx == selection;
        let marker = if selected && active {
            "▶ "
        } else if lang == state.config.language {
            "● "
        } else {
            "  "
        };
        let btn =
            Button::new(format!("{marker}{lang}")).on_press(move || AppMsg::SelectLanguage(lang));
        let btn = if selected { btn } else { btn.secondary() };
        ctx.component(
            format!("lang_{idx}"),
            btn,
            Rect::new(list_box.x, list_box.y + row as u16, list_box.width, 1),
        );
    }
}
