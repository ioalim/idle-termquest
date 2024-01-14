#![allow(dead_code)]

use std::{rc::Rc, time::{Duration, Instant}};

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
        components::{Command, EntityList, Turn, Component},
        consts::{ACCENT, PRIMARY},
        entities::{enemy::Enemy, hero::Hero, Entity, Id},
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
    current_turn: Option<Id>,
    selected_widget: StateWidget,
    is_in_a_widget: bool,
    command: Command,
    log: Vec<String>,
    turn: Turn,
    timer: Instant,
}

impl InGame {
    pub fn new() -> Self {
        InGame {
            heroes: EntityList::new(),
            enemies: EntityList::new(),
            current_turn: None,
            selected_widget: StateWidget::Command,
            is_in_a_widget: false,
            command: Command::new(),
            log: Vec::new(),
            turn: Turn::new(),
            timer: Instant::now()
        }
    }

    fn handle_nav(&mut self, e: &Event) {
        if self.is_in_a_widget {
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
        let maybe_next_widget = match direction {
            NavDirection::Up => {
                match self.selected_widget {
                    StateWidget::Command => Some(StateWidget::Log),
                    StateWidget::Log     => Some(StateWidget::Hero),
                    _ => None,
                }
            }
            NavDirection::Down => {
                match self.selected_widget {
                    StateWidget::Log   => Some(StateWidget::Command),
                    StateWidget::Hero  => Some(StateWidget::Log),
                    StateWidget::Enemy => Some(StateWidget::Log),
                    StateWidget::Turn  => Some(StateWidget::Log),
                    _ => None,
                }
            }
            NavDirection::Right => {
                match self.selected_widget {
                    StateWidget::Hero => Some(StateWidget::Turn),
                    StateWidget::Turn => Some(StateWidget::Enemy),
                    _ => None,
                }
            }
            NavDirection::Left => {
                match self.selected_widget {
                    StateWidget::Enemy => Some(StateWidget::Turn),
                    StateWidget::Turn  => Some(StateWidget::Hero),
                    _ => None,
                }
            }
        };

        if let Some(next_wid) = maybe_next_widget {
            self.selected_widget = next_wid;
        }
    }
}

impl State for InGame {
    fn init(&mut self) {
        self.heroes.entities.push(Rc::new(Hero::new()));
        self.heroes.entities.push(Rc::new(Hero::new()));
        self.heroes.entities.push(Rc::new(Hero::new()));
        self.enemies.entities.push(Rc::new(Enemy::new()));
        self.enemies.entities.push(Rc::new(Enemy::new()));
        self.enemies.entities.push(Rc::new(Enemy::new()));

        let mut entities: Vec<Rc<dyn Entity>> = Vec::new();
        for (h, e) in self.heroes.entities.iter().zip(self.enemies.entities.iter()) {
            entities.push(h.clone());
            entities.push(e.clone());
        }

        self.turn.set_entities(entities);

        self.current_turn = self.turn.get_current_turn();

        self.timer = Instant::now();
    }

    fn update(&mut self, ctx: &mut Context) -> Option<StateType> {
        self.turn.update_next_round_order();
        if self.timer.elapsed() >= Duration::from_secs(2) {
            self.timer = Instant::now();
            self.current_turn = self.turn.pop();
            ctx.push_log(format!("Current turn: {:?}\n", self.current_turn));
        }
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
        self.turn.render(
            " Turns ",
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
            " Command ",
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
                            'q' if !self.command.is_entered() => {
                                ctx.should_quit = true;
                            }
                            ':' if !self.command.is_entered() => {
                                self.selected_widget = StateWidget::Command;
                                self.command.enter();
                            }
                            _ => (),
                        }
                    }
                    KeyCode::Enter => {
                        if !self.is_in_a_widget {
                            match self.selected_widget {
                                StateWidget::Hero => self.heroes.enter(),
                                StateWidget::Enemy => self.enemies.enter(),
                                StateWidget::Turn => self.turn.enter(),
                                StateWidget::Log => (),
                                StateWidget::Command => self.command.enter(),
                            }

                            if self.selected_widget != StateWidget::Log {
                                self.is_in_a_widget = true;
                            } 
                        } else {
                            match self.selected_widget {
                                StateWidget::Command => {
                                    if self.command.is_entered() {
                                        if let Some(command) = self.command.execute() {
                                            self.log.push(command);
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if self.selected_widget == StateWidget::Command && self.command.is_entered()
                        {
                            self.command.pop()
                        };
                    }
                    KeyCode::Delete => {
                        //self.input.pop_word()
                    }
                    KeyCode::Tab if self.selected_widget == StateWidget::Command => {
                        self.command.exit();
                    }
                    KeyCode::Esc => {
                        if self.is_in_a_widget {
                            match self.selected_widget {
                                StateWidget::Hero => self.heroes.exit(),
                                StateWidget::Enemy => self.enemies.exit(),
                                StateWidget::Turn => self.turn.exit(),
                                StateWidget::Log => (),
                                StateWidget::Command => self.command.exit(),
                            }
                            self.is_in_a_widget = false;
                        } 
                    }
                    _ => (),
                }
                ctx.push_log(format!("{:?}\n", k.code));
            }
            _ => (),
        }
        self.handle_nav(&event);
        if self.is_in_a_widget {
            match self.selected_widget {
                StateWidget::Hero => self.heroes.handle_event(&event),
                StateWidget::Enemy => self.enemies.handle_event(&event),
                StateWidget::Turn => self.turn.handle_event(&event),
                StateWidget::Log => (),
                StateWidget::Command => self.command.handle_event(&event),
            }
        }
        None
    }

    fn get_type(&self) -> StateType {
        StateType::InGame
    }

    fn destroy(&mut self) {}
}
