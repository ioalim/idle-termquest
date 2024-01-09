#![allow(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
use crossterm::event::KeyCode;
use ratatui::{Frame, layout::{Layout, Direction, Constraint, Rect, Alignment}, widgets::Paragraph, style::{Stylize, Color}};

use crate::{Event, Context};

use super::{State, StateType};

pub struct Welcome {
}

impl Welcome {
    pub fn new() -> Self {
        Welcome {}
    }
}

impl State for Welcome {
    fn init(&mut self) {

    }

    fn update(&mut self, _ctx: &mut Context) -> Option<StateType> {

        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Min(1),
                Constraint::Max(1),
            ])
            .split(area);
        let padding_top = ((area.height - 1) as f32 / 2.).floor() as u16;
        let padding_bot = ((area.height - 1) as f32 / 2.).ceil() as u16;
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(padding_top),
                Constraint::Length(1),
                Constraint::Length(padding_bot),
            ])
            .split(main_layout[0]);
        frame.render_widget(
            Paragraph::new("Welcome")
                .alignment(Alignment::Center)
                .fg(Color::Cyan),
                //.block(Block::default().
                //       fg(Color::Cyan)
                //       .borders(Borders::ALL)), 
                layout[1]
        );
        frame.render_widget(
            Paragraph::new("Press 'q' to close")
                .alignment(Alignment::Left)
                .fg(Color::Cyan),
            main_layout[1]
        );
        frame.render_widget(
            Paragraph::new("Press 'enter' continue")
                .alignment(Alignment::Right)
                .fg(Color::Cyan),
            main_layout[1]
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
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        None
    }

    fn get_type(&self) -> StateType {
        StateType::Welcome
    }
}
