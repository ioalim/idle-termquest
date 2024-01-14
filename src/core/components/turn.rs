use std::{collections::VecDeque, ops::Deref, rc::Rc};

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Margin, Rect},
    style::Style,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
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
pub struct Turn {
    selected_item_idx: usize,
    paragraph_offset: usize,
    current_round_order: VecDeque<Id>,
    next_round_order: Vec<Rc<dyn Entity>>,
    enter: bool,
}

impl Turn {
    pub fn new() -> Self {
        Self {
            selected_item_idx: 0,
            paragraph_offset: 0,
            current_round_order: VecDeque::new(),
            next_round_order: Vec::new(),
            enter: false,
        }
    }

    pub fn set_entities(&mut self, next_round_order: Vec<Rc<dyn Entity>>) {
        self.next_round_order = next_round_order;
        self.next_round_order
            .sort_by(|a, b| a.stat().spd.cmp(&b.stat().spd));
        self.current_round_order = self.next_round_order.iter().map(|e| e.id()).collect();
    }

    pub fn update_next_round_order(&mut self) {
        self.next_round_order
            .sort_by(|a, b| a.stat().spd.cmp(&b.stat().spd));
    }

    pub fn pop(&mut self) -> Option<Id> {
        if self.current_round_order.len() == 1 {
            self.goto_next_round();
            self.get_current_turn()
        } else {
            self.current_round_order.pop_front();
            let last_idx = self.current_round_order.len() + self.next_round_order.len() - 1;
            if self.selected_item_idx > last_idx {
                self.selected_item_idx = last_idx;
            }
            self.current_round_order.get(0).copied()
        }
    }

    pub fn get_current_turn(&self) -> Option<Id> {
        self.current_round_order.get(0).copied()
    }

    pub fn goto_next_round(&mut self) {
        self.current_round_order.pop_front();
        self.current_round_order = self.next_round_order.iter().map(|e| e.id()).collect();
        let last_idx = self.current_round_order.len() + self.next_round_order.len() - 1;
        if self.selected_item_idx > last_idx {
            self.selected_item_idx = last_idx;
        }
    }
}

impl Component for Turn {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => match c {
                    'k' if self.selected_item_idx.gt(&0) => self.selected_item_idx -= 1,
                    'j' if self
                        .selected_item_idx
                        .lt(&(self.current_round_order.len() + self.next_round_order.len())) =>
                    {
                        self.selected_item_idx += 1
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
        if self.selected_item_idx < self.paragraph_offset {
            self.paragraph_offset -= 1;
        } else if self.selected_item_idx - self.paragraph_offset >= 3 {
            self.paragraph_offset += 1;
        }
    }

    fn render(&mut self, title: &str, frame: &mut Frame, area: Rect, selected: bool) {
        let color = if selected { ACCENT } else { PRIMARY };
        let entts_name: Vec<&str> = self
            .current_round_order
            .iter()
            .map(|id| {
                self.next_round_order
                    .iter()
                    .find(|e| e.id() == *id)
                    .unwrap()
                    .info()
                    .name
                    .deref()
            })
            .chain(self.next_round_order.iter().map(|e| e.info().name.deref()))
            .collect();
        let paragraph = Paragraph::new(
            entts_name
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    if self.selected_item_idx == i && self.enter {
                        Line::styled(*name, Style::default().reversed())
                    } else {
                        Line::from(*name)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .fg(color)
        .block(Block::new().fg(color).borders(Borders::ALL).title(title))
        .scroll((self.paragraph_offset as u16, 0));

        frame.render_widget(paragraph, area);

        if !self.enter {
            return;
        }

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            //.begin_symbol(Some("↑"))
            //.end_symbol(Some("↓"));
            .begin_symbol(None)
            .end_symbol(None);
        let mut scrollbar_state =
            ScrollbarState::new(self.current_round_order.len() + self.next_round_order.len())
                .position(self.selected_item_idx);

        frame.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }), // using a inner vertical margin of 1 unit makes the scrollbar inside the block
            &mut scrollbar_state,
        );
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

    fn get_type(&self) -> ComponentType {
        ComponentType::Turn
    }
}

#[test]
fn test_usize() {
    let mut u = 1usize;
    u += 1;
    assert_eq!(u, 2);
    u -= 1;
    assert_eq!(u, 1);
    u -= 1;
    assert_eq!(u, 0);
    //u -= 1; // failed
    //assert_eq!(u, 0);
}
