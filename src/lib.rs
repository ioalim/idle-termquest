use std::{time::{Instant, Duration}, thread};

use anyhow::Result;
use app::App;
use event::{EventHandler, Event};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

mod app;
mod ui;
mod event;
mod tui;
mod update;


pub fn run() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let tick_rate = 250;
    let events = EventHandler::new(tick_rate);
    let mut tui = Tui::new(terminal, events);

    #[cfg(not(target_arch = "wasm32"))]
    tui.enter()?;

    while !app.should_quit {
        let start_time = Instant::now();
        match tui.events.next(&mut app)? {
            Event::Tick => (),
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => (),
            Event::Resize(_, _) => (),
        }

        tui.draw(&mut app)?;

        let elapsed_time = start_time.elapsed();
        if elapsed_time < Duration::from_millis(75) {
            thread::sleep(Duration::from_millis(75) - elapsed_time);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    tui.exit()?;
    Ok(())
}

#[allow(dead_code)]
pub fn run_windows() -> Result<()> {
    Ok(())
}

#[allow(dead_code)]
pub fn run_wasm() -> Result<()> {
    Ok(())
}
