use serde::{self, Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, Clone, PartialEq, Deserialize, Serialize)]
pub enum Relaunch {
    #[serde(rename = "never")]
    Never,

    #[serde(rename = "always")]
    Always,

    #[serde(rename = "on-error")]
    OnError,
}

impl Display for Relaunch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Relaunch::Always => "always",
            Relaunch::OnError => "on-error",
            Relaunch::Never => "never",
        };
        write!(f, "{}", s)
    }
}
