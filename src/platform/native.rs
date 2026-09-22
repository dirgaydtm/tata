use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

use crate::app::App;

pub fn run(app: App) -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read()?
            && app.handle_key(code)
        {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
