use std::rc::Rc;

use ratzilla::{WebGl2Backend, WebRenderer};

use crate::app::App;

pub fn run(app: App) -> Result<(), Box<dyn std::error::Error>> {
    console_error_panic_hook::set_once();
    let mut terminal = ratatui::Terminal::new(WebGl2Backend::new()?)?;
    let state = Rc::new(app);

    let event_state = Rc::clone(&state);
    let _ = terminal.on_key_event(move |event| {
        event_state.handle_key(event.code);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}
