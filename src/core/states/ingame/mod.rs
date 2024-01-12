#![allow(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, List},
    Frame,
};

use crate::{
    core::{
        components::{Command, EntityList},
        consts::{ACCENT, PRIMARY},
        entities::{enemy::Enemy, hero::Hero},
    },
    Context, Event,
};

use super::{State, StateType};

#[derive(PartialEq, Clone, Copy)]
enum StateWidget {
    Hero,
    Enemy,
    Log,
    Command,
}

enum NavDirection {
    Up,
    Down,
    Right,
    Left,
}

pub struct InGame {
    heroes: Vec<Hero>,
    enemies: Vec<Enemy>,
    selected_widget: StateWidget,
    command: Command,
    log: Vec<String>,
}

impl InGame {
    pub fn new() -> Self {
        InGame {
            heroes: Vec::new(),
            enemies: Vec::new(),
            selected_widget: StateWidget::Command,
            command: Command::new(),
            log: Vec::new(),
        }
    }

    fn handle_nav(&mut self, e: Event) {
        if self.command.is_typing() {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        match e.key().map(|k| k.code) {
            Some(KeyCode::Up) => self.nav_to(NavDirection::Up),
            Some(KeyCode::Down) => self.nav_to(NavDirection::Down),
            Some(KeyCode::Right) => self.nav_to(NavDirection::Right),
            Some(KeyCode::Left) => self.nav_to(NavDirection::Left),
            Some(KeyCode::Char(c)) => match c {
                'k' => self.nav_to(NavDirection::Up),
                'j' => self.nav_to(NavDirection::Down),
                'l' => self.nav_to(NavDirection::Right),
                'h' => self.nav_to(NavDirection::Left),
                _ => (),
            },
            _ => (),
        }
    }

    fn nav_to(&mut self, direction: NavDirection) {
        match direction {
            NavDirection::Up => {
                self.selected_widget = match self.selected_widget {
                    StateWidget::Command => StateWidget::Log,
                    StateWidget::Log => StateWidget::Hero,
                    _ => self.selected_widget,
                };
            }
            NavDirection::Down => {
                self.selected_widget = match self.selected_widget {
                    StateWidget::Log => StateWidget::Command,
                    StateWidget::Hero => StateWidget::Log,
                    StateWidget::Enemy => StateWidget::Log,
                    _ => self.selected_widget,
                };
            }
            NavDirection::Right => {
                self.selected_widget = match self.selected_widget {
                    StateWidget::Hero => StateWidget::Enemy,
                    _ => self.selected_widget,
                };
            }
            NavDirection::Left => {
                self.selected_widget = match self.selected_widget {
                    StateWidget::Enemy => StateWidget::Hero,
                    _ => self.selected_widget,
                };
            }
        }
    }
}

impl State for InGame {
    fn init(&mut self) {
        self.heroes.push(Hero::new());
        self.heroes.push(Hero::new());
        self.heroes.push(Hero::new());
        self.enemies.push(Enemy::new());
        self.enemies.push(Enemy::new());
        self.enemies.push(Enemy::new());
    }

    fn update(&mut self, _ctx: &mut Context) -> Option<StateType> {
        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let heroes_count = self.heroes.len() as u16;
        let enemies_count = self.enemies.len() as u16;
        let entity_info_height = if heroes_count > enemies_count && heroes_count + 2 < 9 {
            heroes_count + 2
        } else if enemies_count + 2 < 9 {
            enemies_count + 2
        } else {
            9
        };
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(entity_info_height),
                Constraint::Min(1),
                Constraint::Length(3),
            ],
        )
        .split(area);
        let entity_info_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(layout[0]);
        EntityList::render(
            " Heroes ",
            &self.heroes,
            frame,
            entity_info_layout[0],
            self.selected_widget == StateWidget::Hero,
        );
        EntityList::render(
            " Enemies ",
            &self.enemies,
            frame,
            entity_info_layout[1],
            self.selected_widget == StateWidget::Enemy,
        );
        let log_color = if self.selected_widget == StateWidget::Log {
            ACCENT
        } else {
            PRIMARY
        };
        frame.render_widget(
            List::new(self.log.clone()).fg(log_color).block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(log_color)
                    .title(" Log "),
            ),
            layout[1],
        );
        self.command.render(
            frame,
            layout[2],
            self.selected_widget == StateWidget::Command,
        );
    }

    fn handle_event(&mut self, event: Event, ctx: &mut Context) -> Option<StateType> {
        match event {
            #[cfg(not(target_arch = "wasm32"))]
            Event::Key(k) => {
                match k.code {
                    KeyCode::Char(c) => {
                        if self.selected_widget == StateWidget::Command {
                            if k.modifiers == KeyModifiers::CONTROL {
                                self.command.pop_word();
                            } else {
                                self.command.push(c)
                            }
                        }
                        if c == 'q' && !self.command.is_typing() {
                            ctx.should_quit = true;
                        }
                    }
                    KeyCode::Enter => {
                        if self.selected_widget == StateWidget::Command {
                            if self.command.is_typing() {
                                if let Some(command) = self.command.execute() {
                                    self.log.push(command);
                                }
                            } else {
                                self.command.enter();
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if self.selected_widget == StateWidget::Command && self.command.is_typing()
                        {
                            self.command.pop()
                        };
                    }
                    KeyCode::Delete => {
                        //self.input.pop_word()
                    }
                    KeyCode::Tab if self.selected_widget == StateWidget::Command => {
                        self.command.quit();
                    }
                    _ => (),
                }
                ctx.push_log(format!("{:?}\n", k.code));
            }
            _ => (),
        }
        self.handle_nav(event);
        None
    }

    fn get_type(&self) -> StateType {
        StateType::InGame
    }

    fn destroy(&mut self) {}
}
