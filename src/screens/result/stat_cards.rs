use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};
use ratcn::Theme;

use crate::screens::Ctx;

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme, elapsed: f64) {
    let stats = &ctx.state().session.stats;
    let errors = stats.uncorrected_errors();
    let plain = theme.foreground;
    let error_color = if errors > 0 {
        theme.destructive
    } else {
        theme.primary
    };
    let cards = [
        ("acc", format!("{:.1}%", stats.accuracy()), plain),
        ("time", format!("{elapsed:.1}s"), plain),
        ("raw", format!("{:.1}", stats.raw_cpm(elapsed)), plain),
        ("consistency", format!("{:.1}%", stats.consistency()), plain),
        ("errors", errors.to_string(), error_color),
    ];
    let areas = Layout::horizontal([Constraint::Ratio(1, 5); 5])
        .spacing(1)
        .areas::<5>(area);
    for ((title, value, color), card_area) in cards.into_iter().zip(areas) {
        let card = Paragraph::new(Line::from(vec![
            format!("{title}: ").fg(theme.muted_foreground),
            value.fg(color).bold(),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(theme.border)),
        );
        ctx.paint_widget(card, card_area);
    }
}
