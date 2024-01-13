pub mod entity_list;
use std::fmt::Debug;

pub use entity_list::EntityList;

pub mod command;
pub use command::Command;

pub mod turn;
use ratatui::{Frame, layout::Rect};
pub use turn::Turn;

pub trait Component: Debug {
    fn render(&mut self, title: &str, frame: &mut Frame, area: Rect, selected: bool);
    fn enter(&mut self);
    fn is_entered(&self) -> bool;
    fn exit(&mut self);
    fn get_type(&self) -> ComponentType;
}

pub enum ComponentType {
    EntityList,
    Command,
    Turn,
}

