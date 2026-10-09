use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};
use ratcn::Theme;

use crate::{components::progress::ProgressWidget, data::TestMode, screens::Ctx};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let state = ctx.state();
    let [text_area, bar_row] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);

    let elapsed = state.timer.seconds();
    let shown = match state.config.test_mode {
        TestMode::Countdown(lim) => (f64::from(lim) - elapsed).max(0.0),
        TestMode::FullSnippet => elapsed,
    };
    let primary = Style::new().fg(theme.primary).bold();
    let muted = Style::new().fg(theme.muted_foreground);
    ctx.paint_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(format!(" {shown:>4.1}s "), primary),
            Span::raw("   "),
            Span::styled(
                format!("{:>4.0}", state.session.stats.net_cpm(elapsed)),
                primary,
            ),
            Span::styled(" cpm   ", muted),
            Span::styled(format!("{:>3.0}%", state.session.stats.accuracy()), primary),
            Span::styled(" acc", muted),
        ]))
        .alignment(Alignment::Center),
        text_area,
    );
    ctx.paint_widget(
        ProgressWidget::new(state.session.progress()).themed(&theme),
        bar_row.centered_horizontally(Constraint::Length(48)),
    );
}
