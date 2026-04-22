use serde::Serialize;

use crate::error::Result;

pub fn format_json<T: Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)?)
}

pub fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", format_json(value)?);
    Ok(())
}
