use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: u8,
    pub log: VecDeque<String>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick() {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
            //self.log.push_front(format!("Counter incremented: {}", self.counter));
            //if self.log.len() > 20 {
            //    self.log.pop_back();
            //}
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
            //self.log.push_front(format!("Counter decremented: {}", self.counter));
            //if self.log.len() > 20 {
            //    self.log.pop_back();
            //}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::new();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::new();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}

