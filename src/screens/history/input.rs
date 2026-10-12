use super::view::{available_filters, filtered};
use crate::{
    app::{AppState, KeyCode},
    screens::CurrentScreen,
    utils::cycle,
};

pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Esc => state.go_to(CurrentScreen::Typing),
        KeyCode::Up => scroll(state, -1),
        KeyCode::Down => scroll(state, 1),
        KeyCode::PageUp => scroll(state, -10),
        KeyCode::PageDown => scroll(state, 10),
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

fn scroll(state: &mut AppState, delta: isize) {
    let last = filtered(state).count().saturating_sub(1);
    state.history_view.scroll = state
        .history_view
        .scroll
        .saturating_add_signed(delta)
        .min(last);
}
