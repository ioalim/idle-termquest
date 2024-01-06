use std::{panic, io};

use anyhow::Result;
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture}};

use crate::{event::EventHandler, app::App, ui};


pub type CrosstermTerminal = 
    ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: EventHandler,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let panic_hook = panic::take_hook();
            panic::set_hook(Box::new(move |panic| {
                Self::reset().expect("Failed to reset the terminal");
                panic_hook(panic);
            }));
        }

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

