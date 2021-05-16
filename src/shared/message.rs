use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    List,
    Quit,
}
