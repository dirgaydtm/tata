mod code_view;
mod input;

use ratatui::layout::{Constraint, Layout, Rect};

use crate::screens::Ctx;

pub use input::handle_key;

pub fn declare(ctx: &mut Ctx<'_>, area: Rect) {
    let theme = ctx.state().theme();
    let [_, _, container, _] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .spacing(2)
    .margin(3)
    .areas(area);

    code_view::draw(ctx, container, theme);
}
