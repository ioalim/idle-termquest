use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::consts::{ACCENT, PRIMARY};

pub struct Command {
    content: String,
    enter: bool,
}

impl Command {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            enter: false,
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

    pub fn render(&self, frame: &mut Frame, area: Rect, selected: bool) {
        let color = if selected { ACCENT } else { PRIMARY };
        let input_widget_len = area.width as usize - 3;
        let input_exceed_widget = self.content.len() >= input_widget_len;

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::raw(if input_exceed_widget {
                    let offset = self
                        .content
                        .char_indices()
                        .nth_back(input_widget_len - 1)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.content[offset..].to_owned()
                } else {
                    self.content.to_owned()
                }),
                {
                    if self.enter {
                        Span::styled(" ", Style::default().reversed())
                    } else {
                        Span::styled("", Style::default())
                    }
                },
            ]))
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
                    .title(" Command "),
            ),
            area,
        );
    }

    pub fn execute(&mut self) -> Option<String> {
        let result = self.content.clone();
        self.content.clear();
        Some(result)
    }

    pub fn enter(&mut self) {
        self.enter = true;
    }

    pub fn quit(&mut self) {
        self.enter = false;
    }

    pub fn is_typing(&self) -> bool {
        self.enter
    }

    //pub fn content(&self) -> &String {
    //    &self.content
    //}
}