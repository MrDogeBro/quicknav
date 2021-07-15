use anyhow::Result;

use std::io;

use termion::get_tty;

use super::*;

/// Public facing entry point to the interactive shell
pub fn shell(command: Option<String>) -> Result<i32> {
    // Initialize a new Context and capture Stdin
    let stdin = io::stdin();
    let ctx = Context::new(get_tty()?);

    map_keys(ctx, stdin)
}
