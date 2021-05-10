use anyhow::Result;
use colored::*;
use prettytable::{format, Table};

use crate::config;
use crate::utils::string;

pub fn config(option: Option<String>, new_value: Option<String>) -> Result<i32> {
    let config: config::Config = config::Config::load()?;

    let mut option_list = Table::new();
    option_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    option_list.set_titles(row!["Option", "Current Value"]);

    if let Some(option) = option {
        if let Some(new_value) = new_value {
            let value: &str;

            match option.as_str() {
                "create_missing_directories" => {
                    config.options.create_missing_directories = string::to_bool(new_value)?;
                    value = "create_missing_directories";
                }
                _ => {
                    println!("{}", "Error: Option not found or is not valid. Use quicknav config to view available options.".red());
                    return Ok(1);
                }
            }

            println!(
                "{} {} -> {}",
                "Config value updated:".green(),
                value,
                new_value
            );

            config.update();
            return Ok(0);
        }

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
