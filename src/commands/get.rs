use colored::*;
use std::env::var;
use std::fs;
use std::process::exit;

use crate::config;

pub fn get(location: String) {
    let config: config::Config = config::load_config();

    for shortcut in config.shortcuts {
        if shortcut.calls.iter().any(|c| c == &location) {
            println!("{}", shortcut.location.replace("~", &var("HOME").unwrap()));
            exit(0)
        }
    }

    println!(
        "{}",
        "Error: Navigation shortcut not found. Use quicknav list to view all your shortcuts.".red()
    );
    exit(1)
}
