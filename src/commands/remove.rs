use colored::*;
use std::process::exit;

use crate::config;

pub fn remove(shortcut: String) {
    let mut config: config::Config = config::load_config();
    let mut found_shortcut = false;
    let mut index_to_remove: usize = 0;

    for (i, shortcut_conf) in &mut config.shortcuts.iter().enumerate() {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            found_shortcut = true;
            index_to_remove = i;
        }
    }

    if !found_shortcut {
        println!(
            "{} {} {}",
            "Error: Shortcut with call".red(),
            shortcut.red(),
            "was not found.".red()
        );
        exit(1);
    }

    config.shortcuts.remove(index_to_remove);
    config::update_config(config);
    println!("{} {}", "Shortcut removed:".green(), &shortcut);
    exit(0)
}
