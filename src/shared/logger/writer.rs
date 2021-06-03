use super::config::Config;
use log::{Level, Record};
use std::{
    io::{Error, Write},
    time,
};

pub fn write_log<W>(config: &Config, record: &Record<'_>, writer: &mut W) -> Result<(), Error>
where
    W: Write,
{
    if config.instant.is_some() {
        write_instant_with_level(writer, &config.instant.unwrap(), record.level())?;
    } else {
        write_level(writer, record.level())?;
    }

    write_args(writer, record)?;
    Ok(())
}

#[inline(always)]
fn write_instant_with_level<W>(
    writer: &mut W,
    instant: &time::Instant,
    level: Level,
) -> Result<(), Error>
where
    W: Write,
{
    write!(writer, "{:>5}[{:04}] ", level, instant.elapsed().as_secs())?;
    Ok(())
}

#[inline(always)]
fn write_level<W>(writer: &mut W, level: Level) -> Result<(), Error>
where
    W: Write,
{
    write!(writer, "[{:5}] ", level)?;
    Ok(())
}

#[inline(always)]
fn write_args<W>(writer: &mut W, record: &Record<'_>) -> Result<(), Error>
where
    W: Write,
{
    writeln!(writer, "{}", record.args())?;
    Ok(())
}
