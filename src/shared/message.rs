use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    List,
    Quit,
}
