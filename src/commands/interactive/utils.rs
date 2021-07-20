use anyhow::Result;

use std::{env, fmt, fs, io::Write};

use super::*;

#[derive(Debug)]
/// Enum representing the types we will write to the TTY
pub enum Line {
    /// Represents a char
    Char(char),

    /// Represents a String
    Str(String),
}

impl Line {
    /// Returns the length of the line
    pub fn len(&self) -> u16 {
        match self {
            Line::Str(s) => s.chars().count() as u16,
            _ => 1,
        }
    }
}

impl fmt::Display for Line {
    /// Implements Display Trait for the Line enum
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Line::Str(s) => write!(f, "{}", s)?,
            Line::Char(c) => write!(f, "{}", c)?,
        }

        Ok(())
    }
}

/// Returns Ok(true) if there is NOT a shortcut matching the content of the Context
pub fn validate_name(ctx: &mut Context) -> Result<bool> {
    let name: String = ctx.content.iter().collect();

    for shortcut_conf in &mut ctx.config.shortcuts {
        if shortcut_conf.name.to_lowercase() == name.to_lowercase() {
            return Ok(false);
        }
    }

    ctx.name = Some(name);

    Ok(true)
}

/// Validates the location inputted by the user
pub fn validate_location(ctx: &mut Context) -> Result<()> {
    let location: String = ctx.content.iter().collect();
    let shortcut_location: String;
    let cwd = env::current_dir().unwrap().display().to_string();

    if location == "." {
        shortcut_location = cwd;
    } else if location.starts_with(&env::var("HOME").unwrap()) {
        shortcut_location = str::replace(&location, &env::var("HOME").unwrap(), "~");
    } else if location.starts_with("~") {
        shortcut_location = location;
    } else {
        shortcut_location = str::replace(
            &fs::canonicalize(location)?.display().to_string(),
            &env::var("HOME").unwrap(),
            "~",
        );
    }

    ctx.location = Some(shortcut_location);

    Ok(())
}
