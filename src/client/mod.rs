use std::io::{BufReader, prelude::*};
use std::io::ErrorKind;

mod editor;
mod history;

use crate::server::Message;

use self::{editor::Editor, history::History};
use std::os::unix::net::UnixStream;

fn process_line(line: &str) {
    match line.as_ref() {
        "list" => send_message(Message::List),
        _ => {
            println!("Invalid command");
        }
    }
}

fn send_message(msg: Message) {
    let mut stream = UnixStream::connect("/tmp/taskmaster.sock").unwrap();
    let serialized = serde_json::to_string(&msg).unwrap();

    stream.write_all(serialized.as_bytes()).unwrap();
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("{}", line);
}

pub fn start_client() {
    UnixStream::connect("/tmp/taskmaster.sock").unwrap();
    let mut history = History::new();

    loop {
        match Editor::default().readline(&mut history) {
            Ok(line) => {
                println!("Command: {}", line);
                process_line(&line);
                history.push(line);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => break,
            Err(_) => break,
        }
    }
}
