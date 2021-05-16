use nix::{
    sys::{
        signal::Signal,
        stat::{mode_t, Mode},
    },
    unistd::{Gid, Uid},
};
use serde::{
    de::{Deserializer, Error},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum NixError {
    InvalidMode(mode_t),
    InvalidSignal(String),
}

impl Display for NixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NixError::InvalidMode(mode) => write!(f, "invalid mode {}", mode),
            NixError::InvalidSignal(sig) => write!(f, "invalid signal {}", sig),
        }
    }
}

pub struct SerdeMode;

impl SerdeMode {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Mode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_mode = mode_t::deserialize(deserializer)?;
        Mode::from_bits(raw_mode).ok_or_else(|| D::Error::custom(NixError::InvalidMode(raw_mode)))
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(mode: &Mode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw_mode = mode.bits();
        raw_mode.serialize(serializer)
    }
}

pub struct SerdeSignal;

impl SerdeSignal {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Signal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_signal = String::deserialize(deserializer)?;
        let raw_sig = if raw_signal.starts_with("SIG") {
            raw_signal.clone()
        } else {
            let mut raw_sig = String::from("SIG");
            raw_sig.push_str(&raw_signal);
            raw_sig
        };
        Signal::from_str(&raw_sig)
            .map_err(|_| D::Error::custom(NixError::InvalidSignal(raw_signal)))
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(sig: &Signal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&format!("{}", sig), serializer)
    }
}

pub trait UidGid {
    fn from_raw(id: u32) -> Self;
    fn to_raw(&self) -> u32;
}

impl UidGid for Uid {
    fn from_raw(id: u32) -> Self {
        Uid::from_raw(id)
    }

    fn to_raw(&self) -> u32 {
        self.as_raw()
    }
}

impl UidGid for Gid {
    fn from_raw(id: u32) -> Self {
        Gid::from_raw(id)
    }

    fn to_raw(&self) -> u32 {
        self.as_raw()
    }
}

pub struct SerdeOptionnalUidGid<T>(Option<T>)
where
    T: UidGid;

impl<T> SerdeOptionnalUidGid<T>
where
    T: UidGid,
{
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_id = Option::<u32>::deserialize(deserializer)?;
        Ok(raw_id.map(T::from_raw))
    }

    pub fn serialize<S>(id: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        id.as_ref().map(T::to_raw).serialize(serializer)
    }
}
