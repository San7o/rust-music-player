use std::fs::ReadDir;
use std::fs;
use rodio::Sink;

use crate::list::ContentList;

/// Application.
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub sink: Sink,
    pub songs_list: ContentList,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(s: Sink) -> Self {
        App {
         should_quit: false,
         sink: s,
         songs_list: ContentList::from_dir("/home/santo/Music"),
         //songs_list: ContentList::new(),
        }
   }

    // Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
