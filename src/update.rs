use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use std::fs;
use std::io::sink;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::list::ContentList;

fn play_sound(index: u32, app: &mut App) {

    let mut paths = fs::read_dir(app.songs_list.path.clone()).unwrap();
    
    let path = format!("{}", paths.nth(index as usize).unwrap().unwrap().path().display());


    // Chech if this is a folder
    if Path::new(&path).is_dir() {
        // Go inside
        app.songs_list = ContentList::from_dir(&path);
    }
    else {
    
        // Clears the sink
        app.sink.stop();

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(path).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();

        app.sink.append(source);
        app.sink.play();
        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        //app.sink.sleep_until_end(); 
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
                    play_sound(c.to_digit(10).unwrap(), app);
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
            play_sound(app.songs_list.index as u32, app);
        },
        KeyCode::Left => {
            app.songs_list = ContentList::from_dir(
                format!("{}/..", app.songs_list.path).as_str()
            );
        },

        _ => {}
    };
}
