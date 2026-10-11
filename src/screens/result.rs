mod chart;
mod footer;
mod hero;
mod input;
mod stat_cards;

use ratatui::layout::{Constraint, Layout, Rect};

use crate::screens::{Ctx, layout::CONTENT_WIDTH};

pub use input::handle_key;

pub fn declare(ctx: &mut Ctx<'_>, area: Rect) {
    let state = ctx.state();
    let theme = state.theme();
    let elapsed = state.timer.seconds();
    let net_cpm = state.session.stats.net_cpm(elapsed);

    let [hero_area, chart_area, cards_area, footer_area] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .spacing(1)
    .margin(1)
    .areas(area.centered_horizontally(Constraint::Max(CONTENT_WIDTH)));

    hero::draw(ctx, hero_area, theme, net_cpm);
    chart::draw(ctx, chart_area, theme, net_cpm);
    stat_cards::draw(ctx, cards_area, theme, elapsed);
    footer::draw(ctx, footer_area);
}
