use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::{
    app::AppMsg,
    ratcn::button::{Button, ButtonSize},
    screens::{
        Ctx,
        CurrentScreen::{History, Settings},
    },
};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect) {
    let areas = Layout::horizontal([22, 22, 20].map(Constraint::Length))
        .flex(Flex::Center)
        .spacing(3)
        .areas::<3>(area);
    let buttons = [
        ("rst", "Restart (Tab)", AppMsg::Restart(false)),
        ("set", "Settings (Esc)", AppMsg::SetScreen(Settings)),
        ("his", "History (F3)", AppMsg::SetScreen(History)),
    ];
    for ((id, label, msg), area) in buttons.into_iter().zip(areas) {
        let button = Button::new(label)
            .size(ButtonSize::Large)
            .secondary()
            .on_press(move || msg.clone());
        ctx.component(id, button, area);
    }
}
