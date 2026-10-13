use ratatui::layout::Rect;
use ratcn::Theme;

use crate::{
    app::AppMsg,
    ratcn::button::{Button, ButtonSize},
    screens::{
        Ctx, CurrentScreen,
        layout::{footer_areas, hints},
    },
};

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme) {
    let [info, action] = footer_areas(area);
    ctx.paint_widget(
        hints(
            theme,
            &[
                ("Tab", "Section"),
                ("Arrows/Space", "Change"),
                ("Enter", "Select Lang"),
            ],
        ),
        info,
    );
    ctx.component(
        "close_foot",
        Button::new("Back to Typing (Esc)")
            .size(ButtonSize::Large)
            .secondary()
            .on_press(|| AppMsg::SetScreen(CurrentScreen::Typing)),
        action,
    );
}
