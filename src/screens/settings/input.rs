use crate::{
    app::{AppState, KeyCode},
    data::{Language, save_config},
    screens::CurrentScreen,
};

pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            state.settings_view.query.clear();
            state.settings_view.tab = 0;
            state.go_to(CurrentScreen::Typing);
        }
        _ => language_key(state, key),
    }
}

fn language_key(state: &mut AppState, key: KeyCode) {
    let view = &mut state.settings_view;
    match key {
        KeyCode::Enter => {
            if let Some(&l) = Language::search(&view.query).get(view.selection) {
                state.config.language = l;
                let _ = save_config(&state.config);
                state.restart(false);
            }
        }
        KeyCode::Up => view.selection = view.selection.saturating_sub(1),
        KeyCode::Down => {
            let last = Language::search(&view.query).len().saturating_sub(1);
            view.selection = (view.selection + 1).min(last);
        }
        KeyCode::Backspace => {
            view.query.pop();
            view.selection = 0;
        }
        KeyCode::Char(c) if !c.is_control() => {
            view.query.push(c);
            view.selection = 0;
        }
        _ => {}
    }
}
