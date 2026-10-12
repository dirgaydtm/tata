use super::view::available_filters;
use crate::{
    app::{AppState, KeyCode},
    screens::CurrentScreen,
    utils::cycle,
};

pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Esc => state.go_to(CurrentScreen::Typing),
        KeyCode::Left | KeyCode::Right => {
            let filters = available_filters(&state.history);
            cycle(
                &mut state.history_view.filter,
                &filters,
                matches!(key, KeyCode::Right),
            );
            state.history_view.scroll = 0;
        }
        _ => {}
    }
}
