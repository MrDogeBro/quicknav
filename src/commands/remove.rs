use anyhow::{anyhow, Result};
use colored::*;

use crate::config;

pub fn remove(shortcut: String) -> Result<i32> {
    let mut config: config::Config = config::Config::load()?;
    let mut found_shortcut = false;
    let mut index_to_remove: usize = 0;

    for (i, shortcut_conf) in &mut config.shortcuts.iter().enumerate() {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            found_shortcut = true;
            index_to_remove = i;
        }
    }

    if !found_shortcut {
        return Err(anyhow!(format!(
            "Shortcut with call {} was not found.",
            shortcut,
        )));
    }

    config.shortcuts.remove(index_to_remove);
    config.update()?;
    println!("{} {}", "Shortcut removed:".green(), &shortcut);

    Ok(0)
}
