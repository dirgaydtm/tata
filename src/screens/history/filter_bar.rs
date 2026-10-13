use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};
use ratcn::Theme;

use super::view::available_filters;
use crate::{
    app::{AppMsg, AppState},
    ratcn::tabs::{Tab, Tabs},
    screens::Ctx,
};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let [label, tabs] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
    ctx.paint_widget(
        Paragraph::new("Filter by Language".fg(theme.secondary).bold()),
        label,
    );

    let filters = available_filters(&ctx.state().history);
    ctx.component(
        "history_filter_tabs",
        Tabs::new(
            filters
                .into_iter()
                .map(|opt| Tab::new(opt, opt.map_or("All", <&str>::from))),
        )
        .selection(
            |s: &AppState| Some(s.history_view.filter),
            AppMsg::SetHistoryFilter,
        ),
        tabs,
    );
}
