use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::{
    app::AppMsg,
    components::button::{Button, ButtonSize},
    screens::Ctx,
};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect) {
    let areas = Layout::horizontal([22].map(Constraint::Length))
        .flex(Flex::Center)
        .spacing(3)
        .areas::<1>(area);
    let buttons = [("rst", "Restart (Tab)", AppMsg::Restart(false))];
    for ((id, label, msg), area) in buttons.into_iter().zip(areas) {
        let button = Button::new(label)
            .size(ButtonSize::Large)
            .secondary()
            .on_press(move || msg.clone());
        ctx.component(id, button, area);
    }
}
