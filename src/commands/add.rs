use colored::*;
use std::env;
use std::process::exit;

use crate::config;

pub fn add(shortcut: String, location: String, name: Option<String>, description: Option<String>) {
    let mut config: config::Config = config::load_config();

    for shortcut_conf in &mut config.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            println!(
                "{} {} {}",
                "Error: Shortcut with call".red(),
                &shortcut.red(),
                "already exists.".red()
            );
            exit(1)
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
    } else if !location.starts_with(&env::var("HOME").unwrap()) {
        shortcut_location = format!("{}/{}", cwd, location);
    }

    let new_shortcut = config::Shortcut {
        name: shortcut_name,
        description: shortcut_description,
        location: shortcut_location,
        calls: vec![shortcut.to_string()],
    };

    config.shortcuts.push(new_shortcut);
    config::update_config(config);
    println!("{} {}", "New shortcut added:".green(), &shortcut);
    exit(0)
}
