use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use std::fs;
use std::io::sink;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::list::ContentList;
use log::debug;

/// Plays a new sound from the front of the queue, updating the queue
fn play_sound(app: &mut App) {
   
    if !app.sink.is_paused() && app.sink.empty() && !app.play_deque.is_empty() {
        let path = app.play_deque.front().unwrap();
        app.now_playing = path.clone().split("/").last().unwrap().to_string();

        let opened = File::open(path).unwrap();
        let file = BufReader::new(opened);
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();

        app.sink.append(source);
        app.pop_play_deque();
    }
}

fn append_sound(index: u32, app: &mut App) {

    let mut paths = fs::read_dir(app.songs_list.path.clone()).unwrap();
    
    let path = format!("{}", paths.nth(index as usize).unwrap().unwrap().path().display());

    // Chech if this is a folder
    if Path::new(&path).is_dir() {
        // Go inside
        app.songs_list = ContentList::from_dir(&path);
    }
    else {
        app.add_play_deque(path);
    }

}


// The main update function, with all the functions of the application
pub fn update(app: &mut App, key_event: KeyEvent) {
   
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        KeyCode::Char(c) => {

            match c {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    append_sound(c.to_digit(10).unwrap(), app);
                },
                ' ' => {
                    if app.sink.is_paused() {
                        app.sink.play();
                    }
                    else {
                        app.sink.pause();
                    }
                },
                '+' => {
                    if app.sink.volume() <= 2.0 {
                        app.sink.set_volume(app.sink.volume() + 0.05);
                    }
                },
                '-' => {
                    if app.sink.volume() - 0.05 >= 0.0 {
                        app.sink.set_volume(app.sink.volume() - 0.05);
                    }
                    else {
                        app.sink.set_volume(0.0);
                    }
                },
                'r' => {
                    app.pop_play_deque();
                },
                's' => {
                    app.sink.clear();
                    app.sink.play();
                },
                _ => {},
            };
        },
        // Change selected song
        KeyCode::Up => {
            app.songs_list.prev();
        },
        KeyCode::Down=> {
            app.songs_list.next();
        },
        // Play selected sound
        KeyCode::Enter | KeyCode::Right => {
            append_sound(app.songs_list.index as u32, app);
        },
        KeyCode::Left => {
            
            // Only world on linux I suppose
            let new_path: Vec<&str> = app.songs_list.path.split("/").collect();
            let sum_path = &new_path[..new_path.len() - 1].join("/");
            
            app.songs_list = ContentList::from_dir(
                sum_path
            );
        },

        _ => {}
    };

   play_sound(app); 
}
