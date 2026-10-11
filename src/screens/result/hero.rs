use ratatui::{layout::Rect, style::Style};
use ratcn::Theme;
use tui_big_text::{BigText, PixelSize};

use crate::screens::Ctx;

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme, net_cpm: f64) {
    let big_cpm = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .style(Style::new().fg(theme.primary).bold())
        .lines(vec![format!("{net_cpm:.0} CPM").into()])
        .build();
    ctx.paint_widget(big_cpm, area);
}
