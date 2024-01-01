# rust-music-player 

A simple cli music player written in rust and ratatui

![example](./assets/screen1.png)

For now, you have to change the path in src/app.rs 

### Keys

- `up_arrow` scroll up_arrow

- `down_arrow` scroll_down

- `left_arrow` go to previous directory ".."

- `right_arrow` go to directory / play

- `space` play / stop

- `0-9` play the song at index

- `+` raise volume

- `-` lower volume

- `q` or `esc` to quit

### Features

- [x] Scroll through a list of files in a directory
- [x] Play and Pause
- [x] Volume Control
- [x] Move between folders
