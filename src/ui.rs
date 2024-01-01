use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;
use std::fs::ReadDir;
use std::fs;

// Thisis the main function that renders stuff in the UI
pub fn render(app: &mut App, f: &mut Frame) {
    
    // Printing all the paths
    let mut path_string = String::new();
    for (i, path) in app.songs_list.list.iter().enumerate() {
        let tmp_path = path.split("/").last().unwrap();
        if i == app.songs_list.index {
            path_string = format!("{}>> - {}\n", path_string, tmp_path);
        }
        else {
            path_string = format!("{}   - {}\n", path_string, tmp_path);
        }
    }    

    let volume = app.sink.volume() * 100.0;

    // Starting with Ratatui
  f.render_widget(
    Paragraph::new(format!("\n  Volume: {}%\n\n{}", volume.floor(), path_string))
    .block(
      Block::default()
        .title("Rusty Music Player")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Blue))
    .alignment(Alignment::Left),
    f.size(),
  );

}
