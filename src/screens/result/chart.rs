use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Bar, Block, BorderType},
};
use ratcn::Theme;

use crate::{components::barchart::BarChartWidget, screens::Ctx};

/// One bar per second, averaged into buckets when there are more seconds than room.
fn chart_bars(samples: &[f64], fallback: f64, max_bars: usize) -> Vec<Bar<'static>> {
    if samples.is_empty() {
        return vec![
            Bar::default()
                .value(fallback.round() as u64)
                .label(Line::from("CPM")),
        ];
    }
    let count = samples.len().min(max_bars);
    (0..count)
        .map(|b| {
            let start = b * samples.len() / count;
            let end = ((b + 1) * samples.len() / count).max(start + 1);
            let chunk = &samples[start..end];
            let avg = chunk.iter().sum::<f64>() / chunk.len() as f64;
            Bar::default()
                .value(avg.round() as u64)
                .label(Line::from(format!("{end}s")))
        })
        .collect()
}

pub fn draw(ctx: &mut Ctx<'_>, area: Rect, theme: Theme, net_cpm: f64) {
    let samples = ctx.state().session.stats.samples();
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme.border))
        .title(" CPM Over Time")
        .title_style(Style::new().fg(theme.foreground).bold());
    let inner = block.inner(area);
    ctx.paint_widget(block, area);

    let max_bars = (inner.width.saturating_sub(1) / 4).max(3) as usize;
    let bars = chart_bars(samples, net_cpm, max_bars);
    let barchart = BarChartWidget::new(bars)
        .themed(&theme)
        .bar_width(3)
        .bar_gap(1);
    ctx.paint_widget(barchart, inner);
}
