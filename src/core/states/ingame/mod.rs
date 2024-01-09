#![allow(dead_code)]

use ratatui::{Frame, layout::Rect, widgets::Paragraph, style::{Stylize, Color}};

use crate::{Event, Context};

use super::{State, StateType};

pub struct InGame {
}

impl InGame {
    pub fn new() -> Self {
        InGame {}
    }
}

impl State for InGame {
    fn init(&mut self) {
        
    }

    fn update(&mut self, _ctx: &mut Context) -> Option<StateType> {
        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Hi").fg(Color::Cyan), 
            area
        );
    }

    fn handle_event(&mut self, _event: Event, _ctx: &mut Context) -> Option<StateType> {
        None
    }

    fn get_type(&self) -> StateType {
        StateType::InGame
    }
}

