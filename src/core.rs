import std::fs::File;
import std::io::{Write, BufReader, BufRead};
import std::path::Path;

pub struct RnvCore {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub filename: Option<String>,
    pub prompt_mode: bool,
    pub prompt_message: String,
    pub prompt_input: String,
}

impl RnvCore {
    pub fn new(filename: Option<String>) -> Self {
        let mut lines = Vec::new();
        if let Some(ref f) = filename {
            if Path::new(f).exists() {
                if let Ok(file) = File::open(f) {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        if let Ok(l) = line {
                            lines.push(l);
                        }
                    }
                }
            }
        }
        if lines.is_empty() {
            lines.push(String::new());
        }
        Self {
            lines,
            cursor_x: 0,
            cursor_y: 0,
            filename,
            prompt_mode: false,
            prompt_message: String::new(),
            prompt_input: String::new(),
        }
    }

    pub fn save_file(&mut self) -> Result<(), std::io::Error> {
        if let Some(ref f) = self.filename {
            let mut file = File::create(f)?;
            for (i, line) in self.lines.iter().enumerate() {
                file.write_all(line.as_bytes())?;
                if i < self.lines.len() - 1 {
                    file.write_all(b"\n")?;
                }
            }
        }
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) {
        if self.prompt_mode {
            self.prompt_input.insert(self.cursor_x, c);
            self.cursor_x += 1;
        } else {
            self.lines[self.cursor_y].insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.prompt_mode {
            if self.cursor_x > 0 {
                self.prompt_input.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            }
        } else if self.cursor_x > 0 {
            self.lines[self.cursor_y].remove(self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_x = self.lines[self.cursor_y - 1].len();
            let current_line = self.lines.remove(self.cursor_y);
            self.lines[self.cursor_y - 1].push_str(&current_line);
            self.cursor_y -= 1;
        }
    }

    pub fn insert_newline(&mut self) {
        if !self.prompt_mode {
            let remainder = self.lines[self.cursor_y].split_off(self.cursor_x);
            self.lines.insert(self.cursor_y + 1, remainder);
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        if self.prompt_mode {
            let len = self.prompt_input.len();
            self.cursor_x = ((self.cursor_x as i32 + dx).max(0).min(len as i32)) as usize;
        } else {
            let new_y = (self.cursor_y as i32 + dy).max(0).min(self.lines.len() as i32 - 1) as usize;
            self.cursor_y = new_y;
            let len = self.lines[self.cursor_y].len();
            self.cursor_x = ((self.cursor_x as i32 + dx).max(0).min(len as i32)) as usize;
        }
    }
}
