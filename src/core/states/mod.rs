use ratatui::{Frame, layout::Rect};

pub mod welcome;
pub use welcome::Welcome;

pub mod ingame;
pub use ingame::InGame;

use crate::{Event, Context};

pub trait State {
    fn init(&mut self);
    fn update(&mut self, ctx: &mut Context) -> Option<StateType>;
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event, ctx: &mut Context) -> Option<StateType>;
    fn get_type(&self) -> StateType;
}

#[derive(Debug)]
pub enum StateType {
    Welcome,
    InGame
}

