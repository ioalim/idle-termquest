use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        },
        KeyCode::Right | KeyCode::Char('k') => app.increment_counter(),
        KeyCode::Left | KeyCode::Char('j') => app.decrement_counter(),
        _ => ()
    }
}
