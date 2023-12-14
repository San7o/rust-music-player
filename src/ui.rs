use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;
use std::fs::ReadDir;
use std::fs;

pub fn render(app: &mut App, f: &mut Frame) {

    let paths = fs::read_dir("./").unwrap();
    let mut path_string = String::new();
    let mut counter = 0;
    for path in paths {
        let mut tmp_path = format!("{}", path.unwrap().path().display());
        tmp_path = (&tmp_path[2..]).to_string();
        path_string = format!("{}{} - {}\n", path_string, counter, tmp_path);
        counter += 1;
    }    

    /*
    f.render_widget(
    Paragraph::new(format!(
      "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
      app.counter
    ))
    .block(
      Block::default()
        .title("Counter App")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Blue))
    .alignment(Alignment::Center),
    f.size(),
  );
  */ 

  f.render_widget(
    Paragraph::new(format!("\n\n{}", path_string))
    .block(
      Block::default()
        .title("Rusty Music Player")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Blue))
    .alignment(Alignment::Center),
    f.size(),
  );

}
