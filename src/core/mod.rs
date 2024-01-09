use anyhow::Result;
#[cfg(not(target_arch = "wasm32"))]
use crossterm::event::KeyCode;

use ratatui::{Frame, layout::{Layout, Direction, Constraint}, widgets::{Paragraph, Block, Borders, Wrap}, style::{Stylize, Color}};

mod states;
use states::{State, Welcome, InGame, StateType};

use crate::{Event, Context};

pub struct App {
    state: Box<dyn State>
}

impl App {
    pub fn new() -> Self {
        App {
            state: Box::new(Welcome::new())
        }
    }

    pub fn init(&mut self) -> Result<()> {

        self.state.init();
        Ok(())
    }

    pub fn handle_event(&mut self, ctx: &mut Context, event: Event) {
        match event {
            #[cfg(not(target_arch = "wasm32"))]
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => ctx.should_quit = true,
                    _ => ()
                }
                self.state.handle_event(Event::Key(key), ctx);
                if let Some(state) = self.state.handle_event(event, ctx) {
                    self.change_state(state);
                }
            },
            event => {
                if let Some(state) = self.state.handle_event(event, ctx) {
                    self.change_state(state);
                }
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.state.update(ctx);
    }

    pub fn render(&mut self, frame: &mut Frame, ctx: &Context) {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(1), Constraint::Max(5)])
            .split(frame.size());
        frame.render_widget(
            Paragraph::new(ctx.get_log().iter().map(|s| s.as_str()).collect::<String>())
                .block(Block::new().borders(Borders::ALL).fg(Color::Cyan).title(" log "))
                .wrap(Wrap { trim: false }), 
            layout[1]);
        self.state.render(frame, layout[0]);
    }

    fn change_state(&mut self, new_state: StateType) {
        match new_state {
            StateType::Welcome
                if !matches!(self.state.get_type(), StateType::Welcome) => {
                    self.state = Box::new(Welcome::new())
            },
            StateType::InGame
                if !matches!(self.state.get_type(), StateType::InGame) => {
                    self.state = Box::new(InGame::new())
            },
            _ => ()
        }
    }
}
