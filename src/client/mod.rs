use std::io::ErrorKind;
use std::io::{prelude::*, BufReader};

mod editor;
mod history;

use crate::server::error;
use crate::shared::message::Message;

use self::{editor::Editor, history::History};
use std::os::unix::net::UnixStream;

type Result<T> = std::result::Result<T, error::Taskmaster>;

fn send_message(msg: &Message) {
    let mut stream = UnixStream::connect("/tmp/taskmaster.sock").unwrap();
    let serialized = serde_json::to_string(&msg).unwrap();

    stream.write_all(serialized.as_bytes()).unwrap();
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn process_line(history: &History, line: &str) -> Result<()> {
    let vec: Vec<&str> = line.split(' ').collect();

    match *vec.get(0).unwrap() {
        "list" => send_message(&Message::List),
        "reload" => send_message(&Message::Reload),
        "history" => history.print(),
        "help" => print_help(),
        "stop-server" => {
            send_message(&Message::Quit);
            return Ok(());
        }
        "start" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Start((*taskname).to_string()));
                }
            }
        }
        "info" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Info((*taskname).to_string()));
                }
            }
        }
        "stop" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Stop((*taskname).to_string()));
                }
            }
        }
        "status" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Status((*taskname).to_string()));
                }
            }
        }
        "restart" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Restart((*taskname).to_string()));
                }
            }
        }
        _ => {
            println!("Invalid command: {}", line);
            return Err(error::Taskmaster::InvalidCmd);
        }
    };
    Ok(())
}

fn print_help() {
    let s = r#"Usage:
        start: start the task <task>
        stop: stop the task <task>
        restart: restart the task <task>
        reload: reload configuration file
        list: list all available tasks
        info: get info on <task>
        history: display previous valid commands
        help: show this help menu
        status: show status of <command>
        stop-server: stop the server
        "#;
    print!("{}", s);
}

pub fn start() {
    if UnixStream::connect("/tmp/taskmaster.sock").is_ok() {
        let mut history = History::new();

        loop {
            match Editor::default().readline(&mut history) {
                Ok(line) => {
                    if process_line(&history, &line).is_ok() {
                        history.push(line);
                    }
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => break,
                Err(_) => break,
            }
        }
    } else {
        eprintln!("Server isn't running");
    }
}
