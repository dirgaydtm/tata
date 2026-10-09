use std::rc::Rc;

use ratzilla::{
    WebGl2Backend, WebRenderer,
    event::{MouseButton, MouseEventKind},
};

use crate::app::App;

pub fn run(app: App) -> Result<(), Box<dyn std::error::Error>> {
    console_error_panic_hook::set_once();
    let mut terminal = ratatui::Terminal::new(WebGl2Backend::new()?)?;
    let state = Rc::new(app);

    let event_state = Rc::clone(&state);
    let _ = terminal.on_key_event(move |event| {
        event_state.handle_key(event.code, event.ctrl);
    });

    let mouse_state = Rc::clone(&state);
    let _ = terminal.on_mouse_event(move |event| {
        if matches!(event.kind, MouseEventKind::ButtonDown(MouseButton::Left)) {
            mouse_state.handle_click(event.col, event.row);
        }
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.tick();
        render_state.render(frame);
    });

    Ok(())
}
