use ratatui::layout::{Constraint, Layout, Rect};

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
