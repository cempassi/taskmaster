use std::io::{self, prelude::*, BufReader, ErrorKind};

mod editor;
mod history;

use crate::shared::{error, message::Message};

use self::{editor::Editor, history::History};
use std::os::unix::net::UnixStream;

type Result<T> = std::result::Result<T, error::Taskmaster>;

fn send_message(msg: &Message) -> io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/taskmaster.sock")?;
    let serialized = serde_json::to_string(&msg).unwrap();

    stream.write_all(serialized.as_bytes())?;
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn process_line(history: &History, line: &str) -> Result<()> {
    let vec: Vec<&str> = line.split(' ').collect();

    match *vec.get(0).unwrap() {
        "list" => send_message(&Message::List)?,
        "reload" => send_message(&Message::Reload)?,
        "history" => history.print(),
        "help" => print_help(),
        "stop-server" => {
            send_message(&Message::Quit)?;
            return Ok(());
        }
        "start" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Start {
                        id: (*taskname).to_string(),
                    })?;
                }
            }
        }
        "info" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Info {
                        id: (*taskname).to_string(),
                    })?;
                }
            }
        }
        "stop" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Stop {
                        id: (*taskname).to_string(),
                    })?;
                }
            }
        }
        "status" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Status {
                        id: (*taskname).to_string(),
                    })?;
                }
            }
        }
        "restart" => {
            if vec.len() > 1 {
                for taskname in vec.iter().skip(1) {
                    send_message(&Message::Restart {
                        id: (*taskname).to_string(),
                    })?;
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
        exit: exit client
        "#;
    print!("{}", s);
}

pub fn start() -> Result<()> {
    if UnixStream::connect("/tmp/taskmaster.sock").is_ok() {
        let mut history = History::new();

        loop {
            match Editor::default().readline(&mut history) {
                Ok(line) if line == "exit" => {
                    log::info!("stopping client, bye");
                    return Ok(());
                }
                Ok(line) => {
                    log::debug!("line={}", line);
                    let res = process_line(&history, &line);
                    if res.is_ok() {
                        history.push(line);
                        continue;
                    }
                    match res {
                        Err(error::Taskmaster::InvalidCmd) => continue,
                        e => return e,
                    }
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => return Ok(()),
                Err(e) => {
                    log::error!("got error {:?}", e);
                    panic!("unexpected error")
                }
            }
        }
    } else {
        log::error!("Server isn't running");
        Err(error::Taskmaster::InvalidConf)
    }
}
