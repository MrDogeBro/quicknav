use anyhow::Result;

use std::io::Write;

use termion::color;
use termion::clear;

use super::*;

/// Initializes the shell
pub fn base_shell(ctx: &mut Context, message: &str) -> Result<()> {
    write!(
        ctx.shell,
        "{}{}{}{}{}{}{}",
        clear::All,
        ctx.goto(1, 1),
        color::Fg(color::Green),
        message,
        color::Fg(color::Reset),
        ctx.goto(1, 2),
        "-".repeat(message.len()),
    )?;
    ctx.flush()?;

    write!(
        ctx.shell,
        "{}{} Ctrl+c{} {}to exit{}{}",
        ctx.goto(ctx.size.0 + 2, ctx.size.1 - 2),
        termion::style::Bold,
        termion::style::Reset,
        color::Fg(color::Red),
        termion::style::Reset,
        color::Fg(color::Reset),
    )?;
    ctx.flush()?;

    Ok(())
}

// The first menu page
pub fn welcome_page(ctx: &mut Context) -> Result<()> {
    write!(ctx.tty, "{}What would you like to do?", ctx.goto(1, 3))?;
    ctx.flush()?;
    ctx.new_line()?;

    write!(
        ctx.tty,
        "{}[1] {}Add a shortcut",
        color::Fg(color::Green),
        color::Fg(color::Reset)
    )?;
    ctx.flush()?;
    ctx.new_line()?;

    write!(
        ctx.tty,
        "{}[2] {}Edit a shortcut",
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    )?;
    ctx.flush()?;
    ctx.new_line()?;

    write!(
        ctx.tty,
        "{}[3] {}Remove a shortcut",
        color::Fg(color::Red),
        color::Fg(color::Reset)
    )?;
    ctx.flush()?;
    ctx.new_line()?;
    ctx.new_line()?;

    ctx.write_line(Line::Str(" >> ".to_owned()))?;
    ctx.goto_ext(ctx.column, ctx.line)?;

    Ok(())
}
