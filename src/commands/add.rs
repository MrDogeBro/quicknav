use anyhow::{anyhow, Result};
use colored::*;
use std::env;

use crate::config;

pub fn add(
    shortcut: String,
    location: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<i32> {
    let mut config: config::Config = config::Config::load()?;

    for shortcut_conf in &mut config.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            return Err(anyhow!(format!(
                "Shortcut with call {} already exists.",
                &shortcut
            )));
        }
    }

    let mut shortcut_name: String = shortcut.to_string();
    let mut shortcut_description: String = shortcut.to_string();
    let mut shortcut_location = location.to_string();
    let cwd = env::current_dir().unwrap().display().to_string();

    if let Some(name) = name {
        shortcut_name = name;
    }

    if let Some(description) = description {
        shortcut_description = description;
    }

    if location == "." {
        shortcut_location = cwd;
    } else if location.starts_with(&env::var("HOME").unwrap()) {
        shortcut_location = str::replace(&location, &env::var("HOME").unwrap(), "~");
    }

    let new_shortcut = config::Shortcut {
        name: shortcut_name,
        description: shortcut_description,
        location: shortcut_location,
        calls: vec![shortcut.to_string()],
    };

    println!("{:?}", new_shortcut);

    config.shortcuts.push(new_shortcut);
    config.update()?;
    println!("{} {}", "New shortcut added:".green(), &shortcut);

    Ok(0)
}
