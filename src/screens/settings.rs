mod input;
mod language_picker;
mod view;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};

use crate::screens::{Ctx, layout::page};

pub use input::handle_key;
pub use view::SettingsView;

pub fn declare(ctx: &mut Ctx<'_>, area: Rect) {
    let theme = ctx.state().theme();
    let [header, main_content, _] = page(area);

    ctx.paint_widget(Paragraph::new("Settings".fg(theme.primary).bold()), header);

    let [left_col, _] = Layout::horizontal([Constraint::Length(25), Constraint::Fill(1)])
        .spacing(2)
        .areas(main_content);
    language_picker::draw(ctx, left_col, theme);
}
