use anyhow::{anyhow, Result};
use std::env::var;
use std::fs::create_dir_all;
use std::path::Path;

use crate::config;

pub fn get(location: String, search: bool) -> Result<i32> {
    let config: config::Config = config::Config::load()?;

    if search {
        let mut possible_shortcuts: Vec<String> = vec![];

        for shortcut in &config.shortcuts {
            if shortcut.calls.iter().any(|c| c.starts_with(&location)) {
                if let Some(call) = shortcut.calls.first() {
                    possible_shortcuts.push(call.to_string());
                }
            }
        }

        println!("{}", possible_shortcuts.join(" "));
        return Ok(0);
    }

    for shortcut in config.shortcuts {
        if shortcut.calls.iter().any(|c| c == &location) {
            let shortcut_location = shortcut.location.replace("~", &var("HOME").unwrap());

            if Path::new(&shortcut_location).exists() {
                println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
                return Ok(0);
            }

            if !config.options.create_missing_directories {
                return Err(anyhow!(format!(
                    "Shortcut location does not exist {}. If you would like quicknav to automatically create missing directories for you, enable the option create_missing_directories in your config file.",
                    &shortcut_location,
                )));
            }

            create_dir_all(&shortcut_location)?;

            println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
            return Ok(0);
        }
    }

    Err(anyhow!(format!(
        "Navigation shortcut not found. Use quicknav list to view all your shortcuts."
    )))
}
