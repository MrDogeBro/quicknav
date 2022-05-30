use anyhow::{anyhow, Result};
use prettytable::{format, Table};

use crate::config;

fn get_relevant_shortcuts(
    shortcut: Option<String>,
    config: config::Config,
) -> Result<Vec<config::Shortcut>> {
    if let Some(shortcut_name) = shortcut {
        for shortcut_conf in config.shortcuts {
            if shortcut_conf.name == shortcut_name {
                return Ok(vec![shortcut_conf]);
            }
        }

        return Err(anyhow!(format!(
            "Shortcut with name {} was not found",
            shortcut_name
        )));
    }

    Ok(config.shortcuts)
}

fn display_in_table(shortcut: Option<String>, config: config::Config) -> Result<i32> {
    let mut shortcut_list = Table::new();
    shortcut_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    shortcut_list.set_titles(row!["Name", "Shortcuts", "Shortcut Location"]);

    for shortcut_conf in get_relevant_shortcuts(shortcut, config)? {
        let calls: String = shortcut_conf.calls.join(", ");
        shortcut_list.add_row(row![shortcut_conf.name, calls, shortcut_conf.location]);
    }

    shortcut_list.printstd();
    Ok(0)
}

fn display_quiet(shortcut: Option<String>, config: config::Config) -> Result<i32> {
    for shortcut_conf in get_relevant_shortcuts(shortcut, config)? {
        println!("{}", shortcut_conf.location);
    }

    Ok(0)
}

pub fn list(shortcut: Option<String>, quiet: bool) -> Result<i32> {
    let config: config::Config = config::Config::load()?;

    if config.shortcuts.is_empty() {
        return Err(anyhow!(
            "You haven't set up any shortcuts yet, get started with quicknav add."
        ));
    }

    if quiet {
        display_quiet(shortcut, config)
    } else {
        display_in_table(shortcut, config)
    }
}
