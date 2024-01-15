use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{core::consts::{ACCENT, PRIMARY}, Event};

use super::{Component, ComponentType};

#[derive(Debug)]
pub struct Command {
    content: String,
    enter: bool,
    cursor_offset: usize,
}

impl Command {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            enter: false,
            cursor_offset: 0,
        }
    }

    pub fn push(&mut self, c: char) {
        if self.enter {
            self.content.push(c);
        }
    }

    pub fn pop(&mut self) {
        if self.enter {
            self.content.pop();
        }
    }

    pub fn pop_word(&mut self) {
        if self.enter {
            let mut deleted_a_char = false;
            while let Some(c) = self.content.pop() {
                if matches!(c, 'a'..='z' | 'A'..='Z') {
                    deleted_a_char = true;
                } else if c == ' ' && deleted_a_char {
                    self.content.push(' ');
                    break;
                }
            }
        }
    }

    pub fn execute(&mut self) -> Option<String> {
        let result = self.content.clone();
        self.content.clear();
        Some(result)
    }
}

impl Component for Command {
    fn handle_event(&mut self, _event: &Event) {
        
    }

    fn render(&mut self, title: &str, frame: &mut Frame, area: Rect, selected: bool) {
        let color = if selected { ACCENT } else { PRIMARY };
        let inner_widget_len = area.width as usize - 3;
        let input_exceed_widget = self.content.len() >= inner_widget_len;

        let rendered_content = if input_exceed_widget {
            let offset = self
                .content
                .char_indices()
                .nth_back(inner_widget_len - 1)
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.content[offset..].to_owned() + " "
        } else {
            self.content.to_owned()
        };

        frame.render_widget(
            Paragraph::new(rendered_content)
            .fg(color)
            .alignment(if input_exceed_widget {
                Alignment::Right
            } else {
                Alignment::Left
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(color)
                    .title(title),
            ),
            area,
        );

        if self.enter {
            frame.set_cursor(
                if input_exceed_widget {
                    area.x + 1 + inner_widget_len as u16
                } else {
                    area.x + 1 + self.content.len() as u16
                },
                area.y + 1,
            );
        }
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Command
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
