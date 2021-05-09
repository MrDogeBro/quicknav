use anyhow::Result;
use colored::*;
use prettytable::{format, Table};

use crate::config;

pub fn config(option: Option<String>) -> Result<i32> {
    let config: config::Config = config::Config::load()?;

    let mut option_list = Table::new();
    option_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    option_list.set_titles(row!["Option", "Current Value"]);

    if let Some(option) = option {
        match option.as_str() {
            "create_missing_directories" => {
                option_list.add_row(row![
                    "create_missing_directories",
                    config.options.create_missing_directories
                ]);
            }
            _ => {
                println!("{}", "Error: Option not found or is not valid. Use quicknav config to view available options.".red());
                return Ok(1);
            }
        }

        option_list.printstd();
        return Ok(0);
    }

    option_list.add_row(row![
        "create_missing_directories",
        config.options.create_missing_directories
    ]);

    option_list.printstd();
    Ok(0)
}
