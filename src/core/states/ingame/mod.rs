#![allow(dead_code)]

use std::rc::Rc;

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
        components::{Command, EntityList, TurnComponent},
        consts::{ACCENT, PRIMARY},
        entities::{enemy::Enemy, hero::Hero, Entity},
    },
    Context, Event,
};

use super::{State, StateType};

#[derive(PartialEq, Clone, Copy)]
enum StateWidget {
    Hero,
    Enemy,
    Turn,
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
    heroes: EntityList<Hero>,
    enemies: EntityList<Enemy>,
    selected_widget: StateWidget,
    command: Command,
    log: Vec<String>,
    turn: TurnComponent,
}

impl InGame {
    pub fn new() -> Self {
        InGame {
            heroes: EntityList::new(),
            enemies: EntityList::new(),
            selected_widget: StateWidget::Command,
            command: Command::new(),
            log: Vec::new(),
            turn: TurnComponent::new(),
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
                    StateWidget::Turn => StateWidget::Log,
                    _ => self.selected_widget,
                };
            }
            NavDirection::Right => {
                self.selected_widget = match self.selected_widget {
                    StateWidget::Hero => StateWidget::Turn,
                    StateWidget::Turn => StateWidget::Enemy,
                    _ => self.selected_widget,
                };
            }
            NavDirection::Left => {
                self.selected_widget = match self.selected_widget {
                    StateWidget::Enemy => StateWidget::Turn,
                    StateWidget::Turn => StateWidget::Hero,
                    _ => self.selected_widget,
                };
            }
        }
    }
}

impl State for InGame {
    fn init(&mut self) {
        self.heroes .entities.push(Rc::new(Hero::new()));
        self.heroes .entities.push(Rc::new(Hero::new()));
        self.heroes .entities.push(Rc::new(Hero::new()));
        self.enemies.entities.push(Rc::new(Enemy::new()));
        self.enemies.entities.push(Rc::new(Enemy::new()));
        self.enemies.entities.push(Rc::new(Enemy::new()));
    }

    fn update(&mut self, _ctx: &mut Context) -> Option<StateType> {
        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let heroes_count = self.heroes.entities.len() as u16;
        let enemies_count = self.enemies.entities.len() as u16;
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
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ],
        )
        .split(layout[0]);
        self.heroes.render(
            " Heroes ",
            frame,
            entity_info_layout[0],
            self.selected_widget == StateWidget::Hero,
        );
        let mut entities: Vec<Rc<dyn Entity>> = Vec::new();
        for (h, e) in self.heroes.entities.iter().zip(self.enemies.entities.iter()) {
            entities.push(h.clone());
            entities.push(e.clone());
        }
        self.turn.render(
            entities,
            frame,
            entity_info_layout[1],
            self.selected_widget == StateWidget::Turn,
        );
        self.enemies.render(
            " Enemies ",
            frame,
            entity_info_layout[2],
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
                        match c {
                            'q' if !self.command.is_typing() => {
                                ctx.should_quit = true;
                            }
                            ':' if !self.command.is_typing() => {
                                self.selected_widget = StateWidget::Command;
                                self.command.enter();
                            }
                            _ => (),
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
                    KeyCode::Esc if self.selected_widget == StateWidget::Command => {
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
