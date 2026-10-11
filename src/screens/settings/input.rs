use super::view::SETTINGS_TABS;
use crate::{
    app::{AppState, KeyCode},
    data::{CaretStyle, Language, SnippetLength, TestMode, ThemeChoice, save_config},
    screens::CurrentScreen,
    utils::cycle,
};

pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            state.settings_view.query.clear();
            state.settings_view.tab = 0;
            state.go_to(CurrentScreen::Typing);
        }
        KeyCode::Tab => state.settings_view.tab = (state.settings_view.tab + 1) % SETTINGS_TABS,
        _ if state.settings_view.tab == 0 => language_key(state, key),
        KeyCode::Right | KeyCode::Down | KeyCode::Char(' ') | KeyCode::Left | KeyCode::Up => {
            let fwd = matches!(key, KeyCode::Right | KeyCode::Down | KeyCode::Char(' '));
            match state.settings_view.tab {
                1 => {
                    cycle(&mut state.config.test_mode, &TestMode::ALL, fwd);
                    state.restart(false);
                }
                2 => {
                    cycle(&mut state.config.snippet_length, &SnippetLength::ALL, fwd);
                    state.restart(false);
                }
                3 => cycle(&mut state.config.caret_style, &CaretStyle::ALL, fwd),
                4 => state.config.sound_enabled = !state.config.sound_enabled,
                5 => cycle(&mut state.config.theme, &ThemeChoice::ALL, fwd),
                _ => {}
            }
            let _ = save_config(&state.config);
        }
        _ => {}
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
