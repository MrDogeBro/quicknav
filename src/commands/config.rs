use anyhow::Result;
use colored::*;

use crate::config;

pub fn config(option: String) -> Result<i32> {
    let config: config::Config = config::Config::load()?;
    let value;

    match option.as_str() {
        "create_missing_directories" => value = config.options.create_missing_directories,
        _ => {
            println!("{}", "Error: Option not found or is not valid. Use quicknav config to view available options.".red());
            return Ok(1);
        }
    }

    println!("Current value: {}", value);

    Ok(0)
}
