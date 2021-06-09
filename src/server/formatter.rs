use super::{communication::Com, monitor::Status, task::Task};
use std::{
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
    fn send_task(sender: Sender<Com>, name: &str, task: &Task) -> SenderResult;
    fn send_status(sender: Sender<Com>, name: &str, status: Status) -> SenderResult;
    fn send_tasks(
        sender: Sender<Com>,
        tasks: &mut impl Iterator<Item = (String, Task)>,
    ) -> SenderResult;
    fn send_error(sender: Sender<Com>, message: String) -> SenderResult;
}

pub struct Human {}

impl Formatter for Human {
    fn send_tasks(
        sender: Sender<Com>,
        tasks: &mut impl Iterator<Item = (String, Task)>,
    ) -> SenderResult {
        sender.send(Com::Msg(String::from("Available jobs:\n")))?;
        for (name, _task) in tasks {
            sender.send(Com::Msg(format!("    - {}\n", name)))?;
        }
        Ok(())
    }

    fn send_status(sender: Sender<Com>, name: &str, status: Status) -> SenderResult {
        sender.send(Com::Msg(format!("status of {}: {}", name, status)))
    }

    fn send_task(sender: Sender<Com>, name: &str, task: &Task) -> SenderResult {
        sender.send(Com::Msg(format!("Info {}:\n", name)))?;
        sender.send(Com::Msg(task.to_string()))
    }

    fn send_error(sender: Sender<Com>, message: String) -> SenderResult {
        sender.send(Com::Msg(message))
    }
}
