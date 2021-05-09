use anyhow::Result;
use colored::*;
use std::env::var;
use std::fs::create_dir_all;
use std::path::Path;

use crate::config;

pub fn get(location: String) -> Result<i32> {
    let config: config::Config = config::Config::load()?;

    for shortcut in config.shortcuts {
        if shortcut.calls.iter().any(|c| c == &location) {
            let shortcut_location = shortcut.location.replace("~", &var("HOME").unwrap());

            if Path::new(&shortcut_location).exists() {
                println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
                return Ok(0);
            }

            if !config.options.create_missing_directories {
                println!(
                    "{} {}{}",
                    "Error: Shortcut location does not exist".red(),
                    &shortcut_location.red(),
                    ". If you would like quicknav to automatically create missing directories for you, enable the option create_missing_directories in your config file.".red()
                );
                return Ok(1);
            }

            create_dir_all(&shortcut_location)?;

            println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
            return Ok(0);
        }
    }

    println!(
        "{}",
        "Error: Navigation shortcut not found. Use quicknav list to view all your shortcuts.".red()
    );
    Ok(1)
}
