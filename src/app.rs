use ratatui::{
    layout::Alignment,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

#[cfg(target_arch = "wasm32")]
pub use ratzilla::event::KeyCode;

#[cfg(not(target_arch = "wasm32"))]
pub use crossterm::event::KeyCode;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title(" Tata ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let platform = if cfg!(target_arch = "wasm32") {
            "Web (WASM via Ratzilla)"
        } else {
            "Native Desktop Terminal (via Crossterm)"
        };

        let text = format!("tata v0.1.0 • {platform}");

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, frame.area());
    }

    pub fn handle_key(&self, key: KeyCode) -> bool {
        matches!(key, KeyCode::Char('q') | KeyCode::Esc)
    }
}
