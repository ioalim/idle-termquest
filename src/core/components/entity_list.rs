use std::rc::Rc;

use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{core::{
    consts::{ACCENT, PRIMARY},
    entities::Entity,
}, Event};

use super::{Component, ComponentType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct EntityList<E: Entity> {
    pub entities: Vec<Rc<E>>,
    enter: bool,
}

impl<E: Entity> EntityList<E> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            enter: false,
        }
    }
}

impl<E: Entity> Component for EntityList<E> {
    fn handle_event(&mut self, event: &Event) {
        
    }

    fn render(&mut self, title: &str, frame: &mut Frame, area: Rect, selected: bool) {
        let color = if selected { ACCENT } else { PRIMARY };
        frame.render_widget(
            List::new(
                self.entities
                    .iter()
                    .map(|e| {
                        ListItem::new(format!(
                            "{} ({}{})",
                            e.info().name,
                            e.stat().curr_hp,
                            String::from_utf8(vec![0xE2, 0x99, 0xA5]).unwrap()
                        ))
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

