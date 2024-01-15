use std::rc::Rc;

use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{
    core::{
        consts::{ACCENT, PRIMARY},
        entities::{Entity, Id},
    },
    Event,
};

use super::{Component, ComponentType};

#[derive(Debug)]
pub struct EntityList<E: Entity> {
    pub entities: Vec<Rc<E>>,
    enter: bool,
    selected_item_idx: usize,
    turn: Option<Id>,
}

impl<E: Entity> EntityList<E> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            enter: false,
            selected_item_idx: 0,
            turn: None,
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

    pub fn set_turn(&mut self, id: Option<Id>) {
        self.turn = id.and_then(|id| self.entities.iter().find(|e| e.id() == id).map(|e| e.id()));
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
                },
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
        let turn_style = Style::default()
            .add_modifier(Modifier::UNDERLINED)
            .underline_color(if selected {
                ACCENT
            } else {
                PRIMARY
            });
        let selected_turn_style = Style::default()
            .reversed()
            .add_modifier(Modifier::UNDERLINED)
            .underline_color(PRIMARY);
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
                        ))
                        .style(
                            match (
                                i == self.selected_item_idx,
                                selected,
                                self.enter,
                                self.turn.map_or(false, |id| id == e.id()),
                            ) {
                                (true, true, true, true) => selected_turn_style,
                                (true, true, true, false) => selected_item_style,
                                (_, _, _, true) => turn_style,
                                _ => default_style,
                            },
                        )
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
