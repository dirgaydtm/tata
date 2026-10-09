use crate::{
    app::{AppState, KeyCode},
    engine::Session,
};

pub fn handle_key(state: &mut AppState, key: KeyCode, ctrl: bool) {
    match (key, ctrl) {
        (KeyCode::Tab, _) => state.restart(false),
        (KeyCode::Backspace, _) => press(state, Session::backspace),
        (KeyCode::Enter, _) => press(state, |session| session.type_char('\n')),
        (KeyCode::Char(c), false) => press(state, |session| session.type_char(c)),
        _ => {}
    }
}

fn press(state: &mut AppState, input: impl FnOnce(&mut Session)) {
    input(&mut state.session);
    state.ensure_started();
}
