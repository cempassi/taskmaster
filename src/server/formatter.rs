use super::{communication::Com, monitor::Status, task::Task};
use serde::Serialize;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::mpsc::{SendError, Sender},
};

#[derive(PartialEq)]
pub enum MessageFormat {
    Human,
    Yaml,
    Json,
}

impl FromStr for MessageFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "human" => Ok(MessageFormat::Human),
            "yaml" => Ok(MessageFormat::Yaml),
            "json" => Ok(MessageFormat::Json),
            _ => Err(format!("Unknown Message format {}", s)),
        }
    }
}

type SenderResult = Result<(), SendError<Com>>;

pub trait Formatter {
    fn send_task(sender: &Sender<Com>, name: &str, task: &Task) -> SenderResult;
    fn send_status(sender: &Sender<Com>, name: &str, status: Status) -> SenderResult;
    fn send_tasks(
        sender: &Sender<Com>,
        tasks: &mut impl Iterator<Item = (String, Task)>,
    ) -> SenderResult;
    fn send_error(sender: &Sender<Com>, message: String) -> SenderResult;
}

pub struct Human;

impl Formatter for Human {
    fn send_tasks(
        sender: &Sender<Com>,
        tasks: &mut impl Iterator<Item = (String, Task)>,
    ) -> SenderResult {
        sender.send(Com::Msg(String::from("Available jobs:\n")))?;
        for (name, _task) in tasks {
            sender.send(Com::Msg(format!("    - {}\n", name)))?;
        }
        Ok(())
    }

    fn send_status(sender: &Sender<Com>, name: &str, status: Status) -> SenderResult {
        sender.send(Com::Msg(format!("status of {}: {}", name, status)))
    }

    fn send_task(sender: &Sender<Com>, name: &str, task: &Task) -> SenderResult {
        sender.send(Com::Msg(format!("Info {}:\n", name)))?;
        sender.send(Com::Msg(task.to_string()))
    }

    fn send_error(sender: &Sender<Com>, message: String) -> SenderResult {
        sender.send(Com::Msg(message))
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Message {
    Error { message: String },
    Status { taskid: String, status: Status },
    Tasks { tasks: HashMap<String, Task> },
    Task { taskid: String, task: Task },
}

impl Message {
    fn from_error(message: String) -> Self {
        Self::Error { message }
    }

    fn from_status(name: String, status: Status) -> Self {
        Self::Status {
            taskid: name,
            status,
        }
    }

    fn from_tasks_iter(tasks: &mut impl Iterator<Item = (String, Task)>) -> Self {
        Self::from_tasks(tasks.collect())
    }

    fn from_tasks(tasks: HashMap<String, Task>) -> Self {
        Self::Tasks { tasks }
    }

    fn from_task(name: String, task: Task) -> Self {
        Self::Task { taskid: name, task }
    }
}

pub struct Json;

impl Formatter for Json {
    fn send_error(sender: &Sender<Com>, message: String) -> SenderResult {
        let raw_msg = serde_json::to_string(&Message::from_error(message)).unwrap();
        sender.send(Com::Msg(raw_msg))
    }

    fn send_status(sender: &Sender<Com>, name: &str, status: Status) -> SenderResult {
        let raw_msg =
            serde_json::to_string(&Message::from_status(name.to_string(), status)).unwrap();
        sender.send(Com::Msg(raw_msg))
    }

    fn send_task(sender: &Sender<Com>, name: &str, task: &Task) -> SenderResult {
        let raw_msg =
            serde_json::to_string(&Message::from_task(name.to_string(), task.clone())).unwrap();
        sender.send(Com::Msg(raw_msg))
    }

    fn send_tasks(
        sender: &Sender<Com>,
        tasks: &mut impl Iterator<Item = (String, Task)>,
    ) -> SenderResult {
        let raw_msg = serde_json::to_string(&Message::from_tasks_iter(tasks)).unwrap();
        sender.send(Com::Msg(raw_msg))
    }
}

pub struct Yaml;

impl Formatter for Yaml {
    fn send_error(sender: &Sender<Com>, message: String) -> SenderResult {
        let raw_msg = serde_yaml::to_string(&Message::from_error(message)).unwrap();
        sender.send(Com::Msg(raw_msg))
    }

    fn send_status(sender: &Sender<Com>, name: &str, status: Status) -> SenderResult {
        let raw_msg =
            serde_yaml::to_string(&Message::from_status(name.to_string(), status)).unwrap();
        sender.send(Com::Msg(raw_msg))
    }

    fn send_task(sender: &Sender<Com>, name: &str, task: &Task) -> SenderResult {
        let raw_msg =
            serde_yaml::to_string(&Message::from_task(name.to_string(), task.clone())).unwrap();
        sender.send(Com::Msg(raw_msg))
    }

    fn send_tasks(
        sender: &Sender<Com>,
        tasks: &mut impl Iterator<Item = (String, Task)>,
    ) -> SenderResult {
        let raw_msg = serde_yaml::to_string(&Message::from_tasks_iter(tasks)).unwrap();
        sender.send(Com::Msg(raw_msg))
    }
}
