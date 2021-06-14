use std::io::{self, Error, ErrorKind, Write};
use termion::{event::Key, input::TermRead, is_tty, raw::IntoRawMode};

use super::history::{Direction, History};

pub struct Editor {
    should_quit: bool,
    newline: bool,
    interactive_mode: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            newline: false,
            interactive_mode: is_tty(&io::stdin()),
        }
    }

    fn process_keypress(
        &mut self,
        line: &mut String,
        history: &mut History,
    ) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;

        match pressed_key {
            Key::Char(c) if c == '\n' => self.newline = true,
            Key::Char(c) => line.push(c),
            Key::Up if line.is_empty() => {
                if let Some(cmd) = history.get(&Direction::Previous) {
                    *line = cmd;
                }
            }
            Key::Down if line.is_empty() => {
                if let Some(cmd) = history.get(&Direction::Next) {
                    *line = cmd;
                }
            }
            Key::Backspace => {
                line.pop();
            }
            Key::Ctrl('d') => self.should_quit = true,
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
        if self.interactive_mode {
            let previous_stdout_mode = io::stdout().into_raw_mode();
            if previous_stdout_mode.is_err() {
                log::warn!("stdout is not available in raw mode, disabling interactive mode");
                self.interactive_mode = false;
                Editor::readline_raw()
            } else {
                self.readline_interactive(history)
            }
        } else {
            Editor::readline_raw()
        }
    }

    fn readline_interactive(&mut self, history: &mut History) -> Result<String, std::io::Error> {
        let mut line = String::new();

        loop {
            refresh_screen().unwrap();
            display_prompt();
            display_line(&line);
            self.process_keypress(&mut line, history)?;
            if self.should_quit {
                return Err(Error::new(ErrorKind::Interrupted, "Interupted"));
            }
            if self.newline {
                println!("\r");
                return Ok(line);
            }
        }
    }

    fn readline_raw() -> Result<String, std::io::Error> {
        log::debug!("read line in raw mode");
        io::stdin()
            .lock()
            .read_line()?
            .map_or_else(|| Err(Error::new(ErrorKind::Interrupted, "Interupted")), Ok)
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
