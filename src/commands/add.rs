use anyhow::{anyhow, Result};
use colored::*;
use std::env;
use std::fs;

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
                "Shortcut with call {} already exists. Consider using {}.",
                &shortcut, format!("quicknav edit {}", &shortcut).yellow()
            )));
        }
    }

    let mut shortcut_name: String = shortcut.to_string();
    let mut shortcut_description: String = shortcut.to_string();
    let shortcut_location: String;
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
    } else {
        shortcut_location = str::replace(
            &fs::canonicalize(location)?.display().to_string(),
            &env::var("HOME").unwrap(),
            "~",
        );
    }

    let new_shortcut = config::Shortcut {
        name: shortcut_name,
        description: shortcut_description,
        location: shortcut_location,
        calls: vec![shortcut.to_string()],
    };

    config.shortcuts.push(new_shortcut);
    config.update()?;
    println!("{} {}", "New shortcut added:".green(), &shortcut);

    Ok(0)
}

pub fn add_call(shortcut: String, call: String) -> Result<i32> {
    let mut config: config::Config = config::Config::load()?;
    let mut found_shortcut = false;
    let mut shortcut_index: usize = 0;

    for (i, shortcut_conf) in &mut config.shortcuts.iter().enumerate() {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            found_shortcut = true;
            shortcut_index = i;
        }
    }

    if !found_shortcut {
        return Err(anyhow!(format!(
            "Shortcut with call {} was not found.",
            shortcut,
        )));
    }

    for shortcut_conf in &mut config.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &call) {
            return Err(anyhow!(format!(
                "Call {} already exists on the shortcut named {}.",
                &call, shortcut_conf.name
            )));
        }
    }

    config.shortcuts[shortcut_index].calls.push(call.to_owned());
    config.update()?;
    println!("{} {}", "New call added:".green(), &call);

    Ok(0)
}
