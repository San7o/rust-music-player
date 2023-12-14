use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use std::fs;
use std::io::sink;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;

fn play_sound(index: u32, app: &mut App) {
    let mut paths = fs::read_dir("./").unwrap();
    let path = format!("{}", paths.nth(index as usize).unwrap().unwrap().path().display());

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    app.sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    app.sink.sleep_until_end(); 
}


pub fn update(app: &mut App, key_event: KeyEvent) {
    
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        KeyCode::Char(c) => {
            if c.is_digit(10) {
                play_sound(c.to_digit(10).unwrap(), app);
            }
        },
        _ => {}
    };
}
