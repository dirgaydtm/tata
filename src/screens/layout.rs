use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
};
use ratcn::Theme;

pub const CONTENT_WIDTH: u16 = 108;

/// header, body and footer rows shared by the settings and history screens
pub fn page(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .spacing(2)
    .margin(3)
    .areas(area.centered_horizontally(Constraint::Max(CONTENT_WIDTH)))
}

/// the key hints line on the left and the back button area on the right
pub fn footer_areas(area: Rect) -> [Rect; 2] {
    let [info, action] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(26)])
        .flex(Flex::Center)
        .areas(area);
    [info.centered_vertically(Constraint::Length(1)), action]
}

/// "Key: what it does  •  Key: what it does" with the keys highlighted
pub fn hints(theme: Theme, pairs: &[(&str, &str)]) -> Paragraph<'static> {
    let mut spans = Vec::new();
    for (i, (key, text)) in pairs.iter().enumerate() {
        if i > 0 {
            spans.push("  •  ".fg(theme.muted_foreground));
        }
        spans.push(key.to_string().fg(theme.primary).bold());
        spans.push(format!(": {text}").fg(theme.muted_foreground));
    }
    Paragraph::new(Line::from(spans))
}
