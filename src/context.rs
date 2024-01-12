use std::collections::VecDeque;

#[derive(Default)]
pub struct Context {
    pub config: Config,
    pub should_quit: bool,
    pub should_suspend: bool,
    log: VecDeque<String>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn push_log(&mut self, text_log: String) {
        self.log.push_back(text_log);
        if self.log.len() > 3 {
            self.log.pop_front();
        }
    }

    pub fn get_log(&self) -> &VecDeque<String> {
        &self.log
    }
}

#[derive(Default)]
pub struct Config {
    pub tick_rate: f64,
    pub frame_rate: f64,
}
