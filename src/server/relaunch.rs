use serde::{self, Deserialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Relaunch {
    Never,
    Always,
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
