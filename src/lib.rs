use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub fn run() -> Result<()> {
            Ok(())
        }
    } else if #[cfg(target_os = "windows")] {
        pub async fn run() -> Result<()> {
            Ok(())
        }
    } else {
        pub async fn run() -> Result<()> {
            enable_raw_mode()?;
            execute!(stdout(), EnterAlternateScreen)?;
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
            terminal.clear()?;

            loop {
                terminal.draw(|frame| {
                    let area = frame.size();
                    frame.render_widget(
                        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                            .white(),
                        area,
                    );
                })?;

                if event::poll(std::time::Duration::from_millis(16))? {
                    if let event::Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press
                            && key.code == KeyCode::Char('q')
                        {
                            break;
                        }
                    }
                }
            }

            disable_raw_mode()?;
            execute!(stdout(), LeaveAlternateScreen)?;
            Ok(())
        }
    }
}
