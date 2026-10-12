mod filter_bar;
mod input;
mod view;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};

use crate::screens::{Ctx, layout::page};

pub use input::handle_key;
pub use view::HistoryView;

pub fn declare(ctx: &mut Ctx<'_>, area: Rect) {
    let theme = ctx.state().theme();
    let [header, main_content, _] = page(area);

    ctx.paint_widget(
        Paragraph::new("Test History".fg(theme.primary).bold()),
        header,
    );

    let [filter_area, _] = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
        .spacing(1)
        .areas(main_content);
    filter_bar::draw(ctx, filter_area, theme);
}
