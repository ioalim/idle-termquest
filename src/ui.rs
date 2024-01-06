use ratatui::{Frame, widgets::{Paragraph, Block, Borders, BorderType}, style::{Style, Color}, layout::{Alignment, Layout, Direction, Constraint}};

use crate::app::App;

// akdjas
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(
            format!(
                "
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press `j` and `k` to increment and decrement the counter respectively.\n\
                Counter: {}
                ",
                app.counter
            ))
            .block(
                Block::new()
                    .title("Counter App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center),
        layout[0]
    );
    frame.render_widget(
        Paragraph::new(app.log.iter().rev().map(AsRef::as_ref).collect::<Vec<&str>>().join("\n"))
            .block(
                Block::new()
                    .title("Log")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Left),
        layout[1]
    );
}
