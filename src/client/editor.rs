use std::io::{self, stdout, Error, ErrorKind, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use super::history::History;

pub struct Editor {
    should_quit: bool,
    newline: bool,
}

impl Editor {
    fn process_keypress(
        &mut self,
        line: &mut String,
        history: &mut History,
    ) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;

        match pressed_key {
            Key::Char(c) if c == '\n' => self.newline = true,
            Key::Up if line.is_empty() => {
                if let Some(cmd) = history.get(1) {
                    *line = cmd;
                }
            }
            Key::Down if line.is_empty() => {
                if let Some(cmd) = history.get(-1) {
                    *line = cmd;
                }
            }
            Key::Char(c) => line.push(c),
            Key::Backspace => {
                line.pop();
            }
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('c') => {
                if line.is_empty() {
                    println!("")
                }
                line.clear();
            }
            _ => (),
        }
        Ok(())
    }

    pub fn readline(&mut self, history: &mut History) -> Result<String, std::io::Error> {
        let _stdout = stdout().into_raw_mode().unwrap();
        let mut line = String::new();

        loop {
            refresh_screen().unwrap();
            display_prompt();
            display_line(&line);
            if let Err(error) = self.process_keypress(&mut line, history) {
                die(error);
            }
            if self.should_quit {
                return Err(Error::new(ErrorKind::Interrupted, "Interupted"));
            }
            if self.newline {
                println!("\r");
                return Ok(line);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            newline: false,
        }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn refresh_screen() -> Result<(), std::io::Error> {
    print!("{}\r", termion::clear::CurrentLine);
    io::stdout().flush()
}

fn display_line(line: &str) {
    print!("{}", line);
    io::stdout().flush().unwrap();
}

fn display_prompt() {
    print!("[~>] ");
    io::stdout().flush().unwrap();
}

fn die(e: std::io::Error) {
    panic!(e);
}
