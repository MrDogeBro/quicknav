use colored::*;
use std::env::var;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;

use crate::config;

pub fn get(location: String) {
    let config: config::Config = config::load_config();

    for shortcut in config.shortcuts {
        if shortcut.calls.iter().any(|c| c == &location) {
            let shortcut_location = shortcut.location.replace("~", &var("HOME").unwrap());

            if Path::new(&shortcut_location).exists() {
                println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
                exit(0);
            }

            if !config.options.create_missing_directories {
                println!(
                    "{} {}{}",
                    "Error: Shortcut location does not exist".red(),
                    &shortcut_location.red(),
                    ". If you would like quicknav to automatically create missing directories for you, enable the option create_missing_directories in your config file.".red()
                );
                exit(1);
            }

            if let Err(e) = create_dir_all(&shortcut_location) {
                println!(
                    "{} {}. Traceback: {}",
                    "Error: Could not create directories for path".red(),
                    &shortcut_location.red(),
                    e.to_string().red()
                );
                exit(1);
            }

            println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
            exit(0);
        }
    }

    println!(
        "{}",
        "Error: Navigation shortcut not found. Use quicknav list to view all your shortcuts.".red()
    );
    exit(1)
}
