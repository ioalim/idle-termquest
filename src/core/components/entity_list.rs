use std::rc::Rc;

use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Stylize, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{core::{
    consts::{ACCENT, PRIMARY},
    entities::Entity,
}, Event};

use super::{Component, ComponentType};

#[derive(Debug)]
pub struct EntityList<E: Entity> {
    pub entities: Vec<Rc<E>>,
    enter: bool,
    selected_item_idx: usize,
}

impl<E: Entity> EntityList<E> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            enter: false,
            selected_item_idx: 0,
        }
    }

    fn select_up(&mut self) {
        if self.selected_item_idx > 0 {
            self.selected_item_idx -= 1;
        }
    }

    fn select_down(&mut self) {
        if self.selected_item_idx < self.entities.len() - 1 {
            self.selected_item_idx += 1;
        }
    }
}

impl<E: Entity> Component for EntityList<E> {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => match c {
                    'k' => self.select_up(),
                    'j' => self.select_down(),
                    _ => (),
                }
                KeyCode::Up => self.select_up(),
                KeyCode::Down => self.select_down(),
                _ => (),
            },
            _ => (),
        }
    }

    fn render(&mut self, title: &str, frame: &mut Frame, area: Rect, selected: bool) {
        let color = if selected { ACCENT } else { PRIMARY };
        let default_style = Style::default();
        let selected_item_style = Style::default().reversed();
        frame.render_widget(
            List::new(
                self.entities
                    .iter()
                    .enumerate()
                    .map(|(i, e)| {
                        ListItem::new(format!(
                            "{} ({}{})",
                            e.info().name,
                            e.stat().curr_hp,
                            String::from_utf8(vec![0xE2, 0x99, 0xA5]).unwrap()
                        )).style(if i == self.selected_item_idx && selected && self.enter {
                            selected_item_style
                        } else {
                            default_style
                        })
                    })
                    .collect::<Vec<ListItem>>(),
            )
            .fg(color)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(color)
                    .title(title),
            ),
            area,
        );
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::EntityList
    }

    fn enter(&mut self) {
        self.enter = true;
    }

    fn is_entered(&self) -> bool {
        self.enter
    }

    fn exit(&mut self) {
        self.enter = false;
    }
}

