use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use std::fs;
use std::io::sink;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;

fn play_sound(index: u32, app: &mut App) {

    // Clears the sink
    app.sink.stop();

    let mut paths = fs::read_dir(app.songs_list.path.clone()).unwrap();
    let path = format!("{}", paths.nth(index as usize).unwrap().unwrap().path().display());

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    app.sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    //app.sink.sleep_until_end(); 
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
        KeyCode::Enter => {
            play_sound(app.songs_list.index as u32, app)
        },

        _ => {}
    };
}
