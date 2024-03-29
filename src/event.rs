#[cfg(not(target_arch = "wasm32"))]
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    #[cfg(not(target_arch = "wasm32"))]
    Key(KeyEvent),
    #[cfg(not(target_arch = "wasm32"))]
    Mouse(MouseEvent),
    Resize(u16, u16),
    None,
}

impl Event {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn key(&self) -> Option<KeyEvent> {
        match self {
            Event::Key(k) => Some(*k),
            _ => None,
        }
    }
}
