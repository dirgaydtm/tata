use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};
use ratcn::Theme;

use super::view::filtered;
use crate::{
    app::{AppMsg, AppState},
    components::scroll_area::ScrollArea,
    screens::Ctx,
    utils::format_timestamp,
};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let mut records: Vec<_> = filtered(ctx.state()).collect();
    records.sort_by_key(|record| std::cmp::Reverse(record.timestamp));

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme.border))
        .title(format!(" Past Sessions ({}) ", records.len()))
        .title_style(Style::new().fg(theme.primary).bold());
    let inner = block.inner(area);
    ctx.paint_widget(block, area);

    let [header_row, divider_row, scroll_view] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    let w_avail = (inner.width.saturating_sub(4) as usize).max(70);
    let w_ts = 20;
    let col_w = (w_avail - w_ts) / 5;

    let header_line = Line::from(vec![
        Span::raw("  "),
        format!("{:<w_ts$}", "Timestamp").fg(theme.primary).bold(),
        format!("{:<col_w$}", "Language").fg(theme.secondary).bold(),
        format!("{:>col_w$}", "Net CPM").fg(theme.primary).bold(),
        format!("{:>col_w$}", "Accuracy").fg(theme.secondary).bold(),
        format!("{:>col_w$}", "Time").fg(theme.foreground).bold(),
        format!("{:>col_w$}", "Errors").fg(theme.destructive).bold(),
    ]);
    ctx.paint_widget(Paragraph::new(header_line), header_row);
    ctx.paint_widget(
        Paragraph::new(format!("  {}", "─".repeat(w_avail)).fg(theme.border)),
        divider_row,
    );

    let lines: Vec<Line> = if records.is_empty() {
        vec![
            "  No tests recorded yet. Complete a typing session to see your progress!"
                .fg(theme.muted_foreground)
                .into(),
        ]
    } else {
        records
            .iter()
            .map(|record| {
                let error_color = if record.error_chars > 0 {
                    theme.destructive
                } else {
                    theme.muted_foreground
                };
                Line::from(vec![
                    Span::raw("  "),
                    format!("{:<w_ts$}", format_timestamp(record.timestamp))
                        .fg(theme.muted_foreground),
                    format!("{:<col_w$}", record.language)
                        .fg(theme.foreground)
                        .bold(),
                    format!("{:>col_w$.1}", record.net_cpm).fg(theme.primary),
                    format!("{:>col_w$}", format!("{:.1}%", record.accuracy)).fg(theme.secondary),
                    format!("{:>col_w$}", format!("{:.1}s", record.duration_seconds))
                        .fg(theme.foreground),
                    format!("{:>col_w$}", record.error_chars).fg(error_color),
                ])
            })
            .collect()
    };

    ctx.component(
        "history_scroll",
        ScrollArea::new(u16::try_from(lines.len()).unwrap_or(u16::MAX))
            .scroll(
                |s: &AppState| s.history_view.scroll as u16,
                |off| AppMsg::ScrollHistory(off as usize),
            )
            .content(move |ctx| {
                let area = ctx.area();
                ctx.paint_widget(Paragraph::new(lines.clone()), area);
            }),
        scroll_view,
    );
}
