use crate::shared::message::Message;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct Communication {
    pub message: Message,
    pub channel: Option<Sender<String>>,
}

impl Communication {
    pub fn new(message: Message, channel: Option<Sender<String>>) -> Communication {
        Communication { message, channel }
    }
}
