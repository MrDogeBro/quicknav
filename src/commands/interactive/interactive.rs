use anyhow::{anyhow, Result};

use std::io;

use termion::get_tty;

use super::*;

/// Public facing entry point to the interactive shell
pub fn shell(command: Option<String>) -> Result<i32> {
    // Initialize a new Context and capture Stdin
    let cmd: String;

    match command {
        Some(c) => {
            match c.to_lowercase().as_str() {
                "add" | "edit" | "remove" => { cmd = c; }
                _ => return Err(anyhow!(format!("{} is not a valid command.", c))),
            }
        }
        None => { cmd = "welcome".to_owned(); }
    }

    let ctx = Context::new(get_tty()?, cmd);
    let stdin = io::stdin();

    map_keys(ctx, stdin)
}
