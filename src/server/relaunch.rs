use serde::{self, Deserialize};

#[derive(Debug, Eq, Clone, PartialEq, Deserialize)]
pub enum Relaunch {
    #[serde(rename = "always")]
    Always,

    #[serde(rename = "on-error")]
    OnError,

    #[serde(rename = "never")]
    Never,
}
