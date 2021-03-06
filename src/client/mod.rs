use std::io::ErrorKind;

mod history;
mod editor;

use self::{editor::Editor, history::History};

pub fn start_client() {
    let mut history = History::new();

    loop {
        match Editor::default().readline(&mut history) {
            Ok(line) => {
                println!("Command: {}", line);
                history.push(line);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => break,
            Err(_) => break,
        }
    }
}
