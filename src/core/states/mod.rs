use ratatui::{layout::Rect, Frame};

pub mod welcome;
pub use welcome::Welcome;

pub mod ingame;
pub use ingame::InGame;

use crate::{Context, Event};

pub trait State {
    fn init(&mut self);
    fn update(&mut self, ctx: &mut Context) -> Option<StateType>;
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event, ctx: &mut Context) -> Option<StateType>;
    fn get_type(&self) -> StateType;
    fn destroy(&mut self);
}

#[derive(Debug, PartialEq)]
pub enum StateType {
    Welcome,
    InGame,
}
