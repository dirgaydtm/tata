use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    data::CaretStyle,
    engine::{CharStatus, Session},
    screens::Ctx,
};

use ratcn::Theme;

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let state = ctx.state();
    let code_rect = area.centered(
        Constraint::Max(100),
        Constraint::Length(area.height.clamp(20, 40)),
    );

    let code_lines = format_code_lines(
        &state.session,
        state.config.caret_style,
        theme,
        (code_rect.height.saturating_sub(2)) as usize,
    );
    ctx.paint_widget(
        Paragraph::new(code_lines).block(
            Block::bordered()
                .border_style(Style::new().fg(theme.border))
                .title(format!(" {} ", state.config.language))
                .title_style(Style::new().fg(theme.secondary).bold()),
        ),
        code_rect,
    );
}

fn format_code_lines(
    session: &Session,
    caret: CaretStyle,
    theme: Theme,
    height: usize,
) -> Vec<Line<'static>> {
    let (code, statuses, active) = (session.code(), session.statuses(), session.index());
    let lines: Vec<&[char]> = code.split(|&c| c == '\n').collect();
    let active_l = code[..active.min(code.len())]
        .iter()
        .filter(|&&c| c == '\n')
        .count();
    let scroll = lines
        .len()
        .saturating_sub(height)
        .min(active_l.saturating_sub(height / 3));

    let (c_style, sym, bg) = match caret {
        CaretStyle::Block => (
            Style::new().fg(theme.background).bg(theme.primary),
            " ",
            theme.primary,
        ),
        CaretStyle::Underline => (
            Style::new().fg(theme.primary).underlined(),
            "_",
            theme.background,
        ),
        CaretStyle::Line => (
            Style::new().fg(theme.primary).bg(theme.surface),
            "│",
            theme.background,
        ),
    };

    let dim = Style::new().fg(theme.muted_foreground);
    let style = |pos: usize| match statuses.get(pos).copied().unwrap_or_default() {
        _ if pos == active => c_style.bold(),
        CharStatus::Untyped => dim,
        CharStatus::Correct => Style::new().fg(theme.foreground),
        CharStatus::Incorrect(_) => Style::new().fg(theme.destructive).underlined().bold(),
    };
    let gutter = |n: usize| Span::styled(format!(" {n:>2} │ "), dim);

    let mut pos: usize = lines[..scroll].iter().map(|l| l.len() + 1).sum();
    // past the last line the view keeps going with empty numbered rows
    lines
        .iter()
        .copied()
        .chain(std::iter::repeat(&[][..]))
        .enumerate()
        .skip(scroll)
        .take(height)
        .map(|(idx, line)| {
            let mut spans = vec![gutter(idx + 1)];
            for &ch in line {
                spans.push(Span::styled(ch.to_string(), style(pos)));
                pos += 1;
            }
            if pos == active {
                spans.push(Span::styled(
                    sym,
                    Style::new().fg(theme.primary).bg(bg).bold(),
                ));
            }
            pos += 1;
            Line::from(spans)
        })
        .collect()
}
