use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Message {
    Reload,
    Start { id: String },
    Info { id: String },
    Stop { id: String },
    Status { id: String },
    Restart { id: String },
    List,
    Quit,
}
