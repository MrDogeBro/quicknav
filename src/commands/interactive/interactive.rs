use anyhow::Result;

use std::io;
use std::io::Write;

use termion::get_tty;

use super::*;

/// Public facing entry point to the interactive shell
pub fn shell(command: Option<String>) -> Result<i32> {
    // Initialize a new Context and capture Stdin
    let stdin = io::stdin();
    let mut ctx = Context::new(get_tty()?);

    map_keys(&mut ctx, stdin)
}
