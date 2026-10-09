use std::time::Duration;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyEventKind, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    },
    execute,
};

use crate::app::App;

pub fn run(app: App) -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let _ = execute!(std::io::stdout(), EnableMouseCapture);

    loop {
        app.tick();
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if app.handle_key(code, modifiers.contains(KeyModifiers::CONTROL)) {
                        break;
                    }
                }
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    column,
                    row,
                    ..
                }) => {
                    app.handle_click(column, row);
                }
                _ => {}
            }
        }
    }

    let _ = execute!(std::io::stdout(), DisableMouseCapture);
    ratatui::restore();
    Ok(())
}
