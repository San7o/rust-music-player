use std::fs;

// This exiists 
// https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html

pub struct ContentList {
    pub index: usize,
    pub list: Vec<String>,
    pub path: String,
}

impl ContentList {

    pub fn new() -> Self {
        ContentList {
            index: 0,
            list: Vec::new(),
            path: String::from("."),
        }
    }

    pub fn from_dir(dir: &str) -> Self {
        
        let mut list = Vec::new();

        let paths = fs::read_dir(format!("{}", dir)).unwrap();
        for path in paths {
            let tmp_path = format!("{}", path.unwrap().path().display());
            list.push(tmp_path);
        }

        ContentList {
            index: 0,
            list: list,
            path: dir.to_string(),
        }
    }

    pub fn add(&mut self, s: String) {
        self.list.push(s);
    }

    pub fn next(&mut self) {
        if self.index + 1 >= self.list.len() {
            self.index = 0;
        }
        else {
            self.index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.index <= 0 {
            self.index = self.list.len() - 1;
        }
        else {
            self.index -= 1;
        }
    }

    pub fn get(&self) -> Option<String> {
        if self.index < self.list.len() {
            Some(self.list[self.index].clone())
        }
        else {
            None
        }
    }

}
