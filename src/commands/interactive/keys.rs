use anyhow::Result;

use std::io;
use std::io::Write;

use termion::screen::ToMainScreen;
use termion::event::Key;
use termion::input::TermRead;

use super::*;

/// Loads base shell and begins listening for keystrokes
pub fn map_keys(mut ctx: Context, stdin: io::Stdin) -> Result<i32> {
    shell_base(&mut ctx, "Welcome to the Quicknav interactive shell!")?;

    // Check which page we should load initially
    match ctx.page.as_str() {
        "welcome" => welcome_page(&mut ctx)?,
        "add" => add_page_base(&mut ctx)?,
        "edit" => edit_page_base(&mut ctx)?,
        "remove" => remove_page_base(&mut ctx)?,
        _ => {}
    }

    // Iterate over key presses and act accordingly
    for key in stdin.keys().map(|c| c.unwrap()) {
        match key {
            Key::Ctrl('c') => break,
            Key::Backspace => ctx.backspace()?,
            Key::Char('\n') => ctx.process_input()?,
            Key::Delete => ctx.del()?,
            Key::Left => ctx.left()?,
            Key::Right => ctx.right()?,
            Key::Up => {}, Key::Down => {}
            Key::Char(c) => ctx.write_char(c)?,
            _ => write!(ctx.tty, "**{:?}", key)?,
        }

        ctx.flush()?;
    }

    // Exit the interactive shell
    write!(ctx.tty, "{}", ToMainScreen)?;
    ctx.flush()?;

    Ok(0)
}
