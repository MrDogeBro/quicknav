use anyhow::{anyhow, Result};
use colored::*;
use std::env;
use std::fs;

use crate::config;

pub fn edit(
    shortcut: String,
    location: Option<String>,
    name: Option<String>,
    description: Option<String>,
) -> Result<i32> {

    if let (None, None, None) = (&location, &name, &description) {
        return Err(anyhow!("No data was provided to edit {}.", &shortcut));
    }

    let cwd = env::current_dir().unwrap().display().to_string();
    let mut config = config::Config::load()?;
    let mut res = "Shortcut edited: ".green().to_string();
    let mut valid_shortcut = false;

    for shortcut_conf in &mut config.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {

            valid_shortcut = true;

            match &name {
                Some(n) => res.push_str(&format!("{}", n)),
                _ => res.push_str(&format!("{}", shortcut_conf.name)),
            }

            if let Some(location) = &location {
                if location == "." {
                    shortcut_conf.location = cwd.to_owned();
                } else if location.starts_with(&env::var("HOME").unwrap()) {
                    shortcut_conf.location = str::replace(
                        &location, &env::var("HOME").unwrap(), "~"
                    );
                } else {
                    shortcut_conf.location = str::replace(
                        &fs::canonicalize(location)?.display().to_string(),
                        &env::var("HOME").unwrap(),
                        "~",
                    );
                }

                res.push_str(&format!("\nLocation: {}", &shortcut_conf.location));
            }

            if let Some(description) = &description {
                shortcut_conf.description = description.to_owned();
                res.push_str(&format!("\nDescription: {}", &description));
            }
        }
    }

    if !valid_shortcut {
        return Err(anyhow!(format!("No existing call matching {}.",&shortcut)));
    }

    config.update()?;
    println!("{}", res);

    Ok(0)
}
