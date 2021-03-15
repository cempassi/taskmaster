use std::io::ErrorKind;
use std::io::prelude::*;

mod history;
mod editor;

use self::{editor::Editor, history::History};
use std::os::unix::net::UnixStream;

pub fn start_client() {
    let mut history = History::new();

    loop {
        match Editor::default().readline(&mut history) {
            Ok(line) => {
                let mut stream = UnixStream::connect("/tmp/taskmaster.sock").unwrap();
                println!("Command: {}", line);
                stream.write_all(line.as_bytes()).unwrap();
                history.push(line);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => break,
            Err(_) => break,
        }
    }
}
