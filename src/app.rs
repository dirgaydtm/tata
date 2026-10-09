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
    toast::{Toast, ToasterState},
};
use web_time::Instant;

use crate::{
    components::toast::ToasterWidget,
    data::{Snippet, UserConfig, load_config},
    engine::Session,
    screens::{self},
    utils::Timer,
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
    pub timer: Timer,
    pub focus: FocusState,
    pub record_saved: bool,
    pub toaster: ToasterState<'static>,
    pub created_at: Instant,
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
            timer: Timer::default(),
            focus: FocusState::default(),
            record_saved: false,
            toaster: ToasterState::new(),
            created_at: Instant::now(),
        }
    }
}

impl AppState {
    pub fn theme(&self) -> Theme {
        self.config.theme.to_theme()
    }

    pub fn notify(&mut self, toast: Toast<'static>) {
        let now = self.created_at.elapsed();
        self.toaster.push(toast, now);
    }

    pub fn ensure_started(&mut self) {
        if self.session.has_started() && !self.timer.is_running() {
            self.timer.start();
        }
    }

    pub fn tick(&mut self) {
        let now = self.created_at.elapsed();
        let _ = self.toaster.prune_expired(now);
        if self.timer.is_running() && self.session.has_started() {
            self.session.stats.tick(self.timer.seconds());
        }
        self.update_completion();
    }

    pub fn update_completion(&mut self) {
        if self.record_saved {
            return;
        }
        let elapsed = self.timer.seconds();
        let expired = self.session.has_started()
            && self
                .config
                .test_mode
                .duration()
                .is_some_and(|l| elapsed >= f64::from(l));
        if expired || self.session.is_complete() {
            let net = self.session.stats.net_cpm(elapsed);
            let acc = self.session.stats.accuracy();
            self.notify(
                Toast::success("Test Completed!")
                    .with_description(format!("{net:.0} CPM • {acc:.0}% Acc")),
            );
            self.record_saved = true;
            self.session.complete();
            self.timer.pause();
        }
    }

    pub fn restart(&mut self, same: bool) {
        if !same {
            let data = self.config.language.snippet_data();
            self.snippet = data
                .random(self.config.snippet_length)
                .unwrap_or_else(|| data.snippets[0].clone());
            let lang_name: &'static str = self.config.language.into();
            self.notify(Toast::new("New Snippet Loaded").with_description(lang_name));
        }
        self.session = Session::new(&self.snippet.code);
        self.timer.reset();
        self.record_saved = false;
    }

    /// returns true when the app should quit
    pub fn on_key(&mut self, key: KeyCode, ctrl: bool) -> bool {
        if ctrl && matches!(key, KeyCode::Char('q' | 'Q' | 'c' | 'C')) {
            return true;
        }

        screens::typing::handle_key(self, key, ctrl);
        self.update_completion();
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
    pub fn tick(&self) {
        self.state.borrow_mut().tick();
    }

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

        let state = self.state.borrow();
        let now = state.created_at.elapsed();
        let toaster = ToasterWidget::new(&state.toaster, now).themed(&theme);
        frame.render_widget(toaster, inner_area);
    }

    pub fn handle_key(&self, key: KeyCode, ctrl: bool) -> bool {
        self.state.borrow_mut().on_key(key, ctrl)
    }
}
