use std::{ops::Deref, rc::Rc};

use ratatui::{
    layout::{Margin, Rect},
    style::Style,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::core::{
    consts::{ACCENT, PRIMARY},
    entities::Entity,
};

pub struct Turn {
    vertical_scroll: usize,
}

impl Turn {
    pub fn new() -> Self {
        Self { vertical_scroll: 0 }
    }

    pub fn render(
        &self,
        mut entts: Vec<Rc<dyn Entity>>,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
    ) {
        let color = if selected { ACCENT } else { PRIMARY };
        entts.sort_by(|a, b| a.stat().spd.cmp(&b.stat().spd));
        let paragraph = Paragraph::new(
            entts
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    if self.vertical_scroll == i && selected {
                        Line::styled(e.info().name.deref(), Style::default().reversed())
                    } else {
                        Line::from(e.info().name.deref())
                    }
                })
                .collect::<Vec<_>>(),
        )
        .fg(color)
        .block(
            Block::new()
                .fg(color)
                .borders(Borders::ALL)
                .title(" Turns "),
        )
        .scroll((self.vertical_scroll as u16, 0));

        frame.render_widget(paragraph, area);

        if !selected {
            return;
        }

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            //.begin_symbol(Some("↑"))
            //.end_symbol(Some("↓"));
            .begin_symbol(None)
            .end_symbol(None);
        let mut scrollbar_state = ScrollbarState::new(entts.len()).position(self.vertical_scroll);

        frame.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }), // using a inner vertical margin of 1 unit makes the scrollbar inside the block
            &mut scrollbar_state,
        );
    }
}
