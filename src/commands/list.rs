use anyhow::{anyhow, Result};
use prettytable::{format, Table};

use crate::config;

pub fn list(shortcut: Option<String>) -> Result<i32> {
    if let Some(shortcut) = shortcut {
        let config: config::Config = config::Config::load()?;

        let mut shortcut_list = Table::new();
        shortcut_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        shortcut_list.set_titles(row!["Name", "Shortcuts", "Shortcut Location"]);

        for shortcut_conf in config.shortcuts {
            if shortcut_conf.name.to_lowercase() == shortcut.to_lowercase() {
                let calls: String = shortcut_conf.calls.join(", ");
                shortcut_list.add_row(row![shortcut_conf.name, calls, shortcut_conf.location]);
                shortcut_list.printstd();
                return Ok(0);
            }
        }

        Err(anyhow!(format!(
            "Shortcut with name {} was not found",
            shortcut
        )))
    } else {
        let config: config::Config = config::Config::load()?;

        let mut shortcut_list = Table::new();
        shortcut_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        shortcut_list.set_titles(row!["Shortcuts", "Shortcut Name", "Shortcut Location"]);

        for shortcut_conf in config.shortcuts {
            let calls: String = shortcut_conf.calls.join(", ");
            shortcut_list.add_row(row![calls, shortcut_conf.name, shortcut_conf.location]);
        }

        shortcut_list.printstd();
        Ok(0)
    }
}
