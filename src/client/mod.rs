use std::io::ErrorKind;
use std::io::{prelude::*, BufReader};

mod editor;
mod history;

use crate::server::Message;

use self::{editor::Editor, history::History};
use std::os::unix::net::UnixStream;

fn send_message(msg: Message) {
    let mut stream = UnixStream::connect("/tmp/taskmaster.sock").unwrap();
    let serialized = serde_json::to_string(&msg).unwrap();

    stream.write_all(serialized.as_bytes()).unwrap();
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("{}", line);
}

fn process_line(history: &mut History, line: String) {
    match line.as_ref() {
        "list" => send_message(Message::List),
        "history" => history.print(),
        "quit" => send_message(Message::Quit),
        _ => {
            println!("Invalid command");
        }
    }
    history.push(line);
}

pub fn start_client() {
    if let Ok(_) = UnixStream::connect("/tmp/taskmaster.sock") {
        println!("Starting client");
        let mut history = History::new();

        loop {
            match Editor::default().readline(&mut history) {
                Ok(line) => {
                    println!("Command: {}", line);
                    process_line(&mut history, line);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => break,
                Err(_) => break,
            }
        }
    } else {
        eprintln!("Server isn't running");
    }
}
