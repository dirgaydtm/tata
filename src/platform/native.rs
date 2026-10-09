use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::App;

pub fn run(app: App) -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    loop {
        app.tick();
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            && app.handle_key(code, modifiers.contains(KeyModifiers::CONTROL))
        {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
