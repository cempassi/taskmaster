use std::io::ErrorKind;
use std::io::{prelude::*, BufReader};

mod editor;
mod history;

use crate::server::Message;

use self::{editor::Editor, history::History};
use std::os::unix::net::UnixStream;

fn send_message(msg: &Message) {
    let mut stream = UnixStream::connect("/tmp/taskmaster.sock").unwrap();
    let serialized = serde_json::to_string(&msg).unwrap();

    stream.write_all(serialized.as_bytes()).unwrap();
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn process_line(history: &mut History, line: String) -> bool {
    match line.as_ref() {
        "list" => send_message(&Message::List),
        "history" => history.print(),
        "help" => print_help(),
        "quit" => {
            send_message(&Message::Quit);
            return false;
        }
        _ => {
            println!("Invalid command: {}", line);
        }
    }
    history.push(line);
    true
}

fn print_help() {
    println!("Help is on the way...");
}

pub fn start() {
    if UnixStream::connect("/tmp/taskmaster.sock").is_ok() {
        let mut history = History::new();

        loop {
            match Editor::default().readline(&mut history) {
                Ok(line) => if process_line(&mut history, line) == false {},
                Err(e) if e.kind() == ErrorKind::Interrupted => break,
                Err(_) => break,
            }
        }
    } else {
        eprintln!("Server isn't running");
    }
}
