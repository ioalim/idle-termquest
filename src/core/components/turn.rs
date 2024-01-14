use std::{collections::VecDeque, ops::Deref, rc::Rc};

use ratatui::{
    layout::{Margin, Rect},
    style::Style,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::{core::{
    consts::{ACCENT, PRIMARY},
    entities::{Entity, Id},
}, Event};

use super::{Component, ComponentType};

#[derive(Debug)]
pub struct Turn {
    vertical_scroll: usize,
    current_round_order: VecDeque<Id>,
    next_round_order: Vec<Rc<dyn Entity>>,
    enter: bool,
}

impl Turn {
    pub fn new() -> Self {
        Self {
            vertical_scroll: 0,
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
            self.current_round_order.get(0).copied()
        }
    }

    pub fn get_current_turn(&self) -> Option<Id> {
        self.current_round_order.get(0).copied()
    }

    pub fn goto_next_round(&mut self) {
        self.current_round_order.pop_front();
        self.current_round_order = self.next_round_order.iter().map(|e| e.id()).collect();
    }
}

impl Component for Turn {
    fn handle_event(&mut self, event: &Event) {
        
    }

    fn render(&mut self, title: &str, frame: &mut Frame, area: Rect, selected: bool) {
        let color = if selected { ACCENT } else { PRIMARY };
        let entts_name: Vec<&str> = self.current_round_order.iter().map(|id| {
            self.next_round_order
                .iter()
                .find(|e| e.id() == *id)
                .unwrap().info().name.deref()
        }).chain(self.next_round_order.iter().map(|e| e.info().name.deref())).collect();
        let paragraph = Paragraph::new(
            entts_name
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    if self.vertical_scroll == i && self.enter {
                        Line::styled(*name, Style::default().reversed())
                    } else {
                        Line::from(*name)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .fg(color)
        .block(Block::new().fg(color).borders(Borders::ALL).title(title))
        .scroll((self.vertical_scroll as u16, 0));

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
            ScrollbarState::new(self.next_round_order.len()).position(self.vertical_scroll);

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
