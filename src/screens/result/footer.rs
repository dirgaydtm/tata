use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::{
    app::AppMsg,
    ratcn::button::Button,
    screens::{Ctx, CurrentScreen},
};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect) {
    let [r_next, r_rep, r_hist] = Layout::horizontal([
        Constraint::Length(24),
        Constraint::Length(18),
        Constraint::Length(18),
    ])
    .flex(Flex::Center)
    .spacing(3)
    .areas(area);

    ctx.component(
        "next",
        Button::new("Next Snippet (Space)").on_press(|| AppMsg::Restart(false)),
        r_next,
    );
    ctx.component(
        "rep",
        Button::new("Repeat (Tab)")
            .secondary()
            .on_press(|| AppMsg::Restart(true)),
        r_rep,
    );
    ctx.component(
        "hist",
        Button::new("History (H)")
            .secondary()
            .on_press(|| AppMsg::SetScreen(CurrentScreen::History)),
        r_hist,
    );
}
