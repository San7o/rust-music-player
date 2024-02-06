use std::fs::ReadDir;
use std::fs;
use rodio::Sink;

use crate::list::ContentList;
use std::collections::VecDeque;

/// Application.
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub sink: Sink,
    pub songs_list: ContentList,
    pub play_deque: VecDeque<String>,
    pub now_playing: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(s: Sink) -> Self {
        App {
         should_quit: false,
         sink: s,
         songs_list: ContentList::from_dir("/home/santo/Music"),
         play_deque: VecDeque::new(),
         now_playing: String::new(),
        }
   }

    // Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// memove the front of the queue
    pub fn pop_play_deque(&mut self) {
        if !self.play_deque.is_empty() {
            let _ = self.play_deque.pop_front();
        }
    }

    pub fn add_play_deque(&mut self, s: String) {
        self.play_deque.push_back(s);
    }
    
}

