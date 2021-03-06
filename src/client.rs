use std::io::ErrorKind;

use crate::{editor::Editor, history::History};

pub fn start_client() {
    let mut history = History::new();

    loop {
        match Editor::default().readline() {
            Ok(line) => {
                println!("Command: {}", line);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => break,
            Err(_) => break,
        }
    }
}
