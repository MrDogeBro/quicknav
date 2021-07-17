use anyhow::{anyhow, Result};

use std::io;

use termion::get_tty;

use super::*;

/// Public facing entry point to the interactive shell
pub fn shell(command: Option<String>) -> Result<i32> {
    let page: String;

    // If the user passed a command, begin on the corresponding page
    match command {
        Some(c) => {
            match c.to_lowercase().as_str() {
                "add" | "edit" | "remove" => { page = c; }
                _ => return Err(anyhow!(format!("{} is not a valid command.", c))),
            }
        }
        None => { page = "welcome".to_owned(); }
    }

    // Initialize a new Context, capture Stdin, and listen for keystrokes
    let ctx = Context::new(get_tty()?, page);
    let stdin = io::stdin();
    map_keys(ctx, stdin)
}
