use std::cell::RefCell;

use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, BorderType, Paragraph},
};

#[cfg(not(target_arch = "wasm32"))]
pub use crossterm::event::KeyCode;
#[cfg(target_arch = "wasm32")]
pub use ratzilla::event::KeyCode;

use ratcn::{
    Theme,
    runtime::{FocusState, Ratcn},
};

use crate::{
    data::{Snippet, UserConfig, load_config},
    engine::Session,
    screens::{self},
};

#[derive(Debug, Clone)]
pub enum AppMsg {
    FocusChanged(FocusState),
}

pub struct App {
    state: RefCell<AppState>,
    ratcn: RefCell<Ratcn<AppState, AppMsg>>,
}

pub struct AppState {
    pub config: UserConfig,
    pub snippet: Snippet,
    pub session: Session,
    pub focus: FocusState,
}

impl Default for AppState {
    fn default() -> Self {
        let config = load_config().unwrap_or_default();
        let snippet = config
            .language
            .snippet_data()
            .random(config.snippet_length)
            .unwrap_or_else(|| config.language.snippet_data().snippets[0].clone());
        Self {
            session: Session::new(&snippet.code),
            snippet,
            config,
            focus: FocusState::default(),
        }
    }
}

impl AppState {
    pub fn theme(&self) -> Theme {
        self.config.theme.to_theme()
    }

    /// returns true when the app should quit
    pub fn on_key(&mut self, key: KeyCode, ctrl: bool) -> bool {
        if ctrl && matches!(key, KeyCode::Char('q' | 'Q' | 'c' | 'C')) {
            return true;
        }

        screens::typing::handle_key(self, key, ctrl);
        false
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: RefCell::new(AppState::default()),
            ratcn: RefCell::new(Ratcn::new().focus(|s: &AppState| &s.focus, AppMsg::FocusChanged)),
        }
    }
}

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let theme = self.state.borrow().theme();
        if area.width < 70 || area.height < 18 {
            frame.render_widget(
                Paragraph::new("Ratype needs at least 70 columns × 18 rows.").centered(),
                area,
            );
            return;
        }

        let app_block = Block::bordered()
            .title(" RATYPE - Touch-Type Tool - v0.1.0 ")
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(theme.border))
            .style(Style::new().bg(theme.background));
        let inner_area = app_block.inner(area);
        frame.render_widget(app_block, area);

        self.ratcn
            .borrow_mut()
            .render(frame, inner_area, &self.state.borrow(), &theme, |ctx| {
                screens::typing::declare(ctx, inner_area)
            });
    }

    pub fn handle_key(&self, key: KeyCode, ctrl: bool) -> bool {
        self.state.borrow_mut().on_key(key, ctrl)
    }
}
