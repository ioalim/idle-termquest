use anyhow::Result;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

mod states;
use states::{InGame, State, StateType, Welcome};

mod entities;

mod components;

mod consts;

mod types;

use crate::{Context, Event};

use self::consts::PRIMARY;

pub struct App {
    state: Box<dyn State>,
}

impl App {
    pub fn new() -> Self {
        App {
            state: Box::new(Welcome::new()),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.state.init();
        Ok(())
    }

    pub fn handle_event(&mut self, ctx: &mut Context, event: Event) {
        if let Some(state) = self.state.handle_event(event, ctx) {
            self.change_state(state);
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if let Some(state) = self.state.update(ctx) {
            self.change_state(state);
        }
    }

    pub fn render(&mut self, frame: &mut Frame, ctx: &Context) {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(1), Constraint::Max(5)],
        )
        .split(frame.size());

        self.state.render(frame, layout[0]);

        frame.render_widget(
            Paragraph::new(ctx.get_log().iter().map(|s| s.as_str()).collect::<String>())
                .fg(PRIMARY)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .fg(PRIMARY)
                        .title(" Debug "),
                )
                .wrap(Wrap { trim: false }),
            layout[1],
        );
    }

    fn change_state(&mut self, state: StateType) {
        let maybe_new_state: Option<Box<dyn State>> = match state {
            StateType::Welcome if self.state.get_type() != StateType::Welcome => {
                Some(Box::new(Welcome::new()))
            }
            StateType::InGame if self.state.get_type() != StateType::InGame => {
                Some(Box::new(InGame::new()))
            }
            _ => None,
        };
        if let Some(new_state) = maybe_new_state {
            self.state.destroy();
            self.state = new_state;
            self.state.init();
        }
    }
}
