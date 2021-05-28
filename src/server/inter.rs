use crate::shared::message::Message;

#[derive(Debug)]
pub enum Inter {
    // When we receive a message from the client
    FromClient(Message),

    // Server need to quit
    Quit,

    // Reload the configuration file
    Reload,
}
