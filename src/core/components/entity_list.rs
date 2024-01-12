use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::core::{
    consts::{ACCENT, PRIMARY},
    entities::Entity,
};

#[allow(dead_code)]
pub struct EntityList;

impl EntityList {
    pub fn render<E: Entity>(
        title: &str,
        entts: &Vec<E>,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
    ) {
        let color = if selected { ACCENT } else { PRIMARY };
        frame.render_widget(
            List::new(
                entts
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
}
