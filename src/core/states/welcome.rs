#![allow(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

use crate::{core::consts::PRIMARY, Context, Event};

use super::{State, StateType};

pub struct Welcome {}

impl Welcome {
    pub fn new() -> Self {
        Welcome {}
    }
}

impl State for Welcome {
    fn init(&mut self) {}

    fn update(&mut self, _ctx: &mut Context) -> Option<StateType> {
        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(1), Constraint::Max(1)],
        )
        .split(area);
        let padding_top = ((area.height - 1) as f32 / 2.).floor() as u16;
        let padding_bot = ((area.height - 1) as f32 / 2.).ceil() as u16;
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(padding_top),
                Constraint::Length(1),
                Constraint::Length(padding_bot),
            ],
        )
        .split(main_layout[0]);
        frame.render_widget(
            Paragraph::new("Welcome")
                .alignment(Alignment::Center)
                .fg(PRIMARY),
            //.block(Block::default().
            //       fg(PRIMARY)
            //       .borders(Borders::ALL)),
            layout[1],
        );
        frame.render_widget(
            Paragraph::new("Press 'q' to close")
                .alignment(Alignment::Left)
                .fg(PRIMARY),
            main_layout[1],
        );
        frame.render_widget(
            Paragraph::new("Press 'enter' continue")
                .alignment(Alignment::Right)
                .fg(PRIMARY),
            main_layout[1],
        );
    }

    fn handle_event(&mut self, event: Event, ctx: &mut Context) -> Option<StateType> {
        match event {
            #[cfg(not(target_arch = "wasm32"))]
            Event::Key(key) => {
                ctx.push_log(format!("{:?}\n", key.code));
                match key.code {
                    KeyCode::Enter => {
                        return Some(StateType::InGame);
                    }
                    KeyCode::Char('q') => {
                        ctx.should_quit = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        None
    }

    fn get_type(&self) -> StateType {
        StateType::Welcome
    }

    fn destroy(&mut self) {}
}
