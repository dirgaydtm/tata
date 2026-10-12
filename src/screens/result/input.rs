use crate::{
    app::{AppState, KeyCode},
    screens::CurrentScreen,
};

pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Tab => state.restart(true),
        KeyCode::Char(' ') | KeyCode::Enter => state.restart(false),
        KeyCode::Char('h' | 'H') => state.go_to(CurrentScreen::History),
        KeyCode::Esc => state.go_to(CurrentScreen::Settings),
        _ => {}
    }
}
