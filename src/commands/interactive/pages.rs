use anyhow::Result;

use std::io::Write;

use termion::color;
use termion::clear;

use super::*;

/// Initializes the shell with a top message and bottom quit message
pub fn shell_base(ctx: &mut Context, message: &str) -> Result<()> {
    ctx.purge();
    write!(
        ctx.shell,
        "{}{}{}{}{}",
        clear::All,
        ctx.goto(1, 1),
        message,
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
pub fn welcome_page(ctx: &mut Context, message: &str) -> Result<()> {
    if ctx.check {
        write!(
            ctx.tty,
            "{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Green),
            message, color::Fg(color::Reset)
        )?;
    } else {
        write!(
            ctx.tty,
            "{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Red),
            message, color::Fg(color::Reset)
        )?;
    }
    ctx.flush()?;

    write!(
        ctx.tty,
        "{}{}[1] {}Add a shortcut",
        ctx.goto(1, 5),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    )?;
    ctx.flush()?;
    ctx.goto_ext(1, 6)?;

    write!(
        ctx.tty,
        "{}{}[2] {}Edit a shortcut",
        ctx.goto(1, 6),
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    )?;
    ctx.flush()?;

    write!(
        ctx.tty,
        "{}{}[3] {}Remove a shortcut",
        ctx.goto(1, 7),
        color::Fg(color::Red),
        color::Fg(color::Reset)
    )?;
    ctx.flush()?;

    ctx.goto_ext(1, 9)?;
    ctx.write_line(Line::Str(" >> ".to_owned()))?;
    ctx.line = 9;
    ctx.column = 5;
    ctx.far_right = 5;

    Ok(())
}

pub fn add_page_base(ctx: &mut Context, message: &str) -> Result<()> {
    shell_base(ctx, "Quicknav * Interactive > Add - Name")?;

    if ctx.check {
        write!(
            ctx.tty,
            "{}{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Green),
            message, ctx.goto(5, 5),
            color::Fg(color::Reset),
        )?;
    } else {
        write!(
            ctx.tty,
            "{}{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Red),
            message, ctx.goto(5, 5),
            color::Fg(color::Reset),
        )?;
    }
    ctx.flush()?;

    ctx.page = "add".to_owned();
    ctx.line = 5;
    ctx.column = 5;
    ctx.far_right = 5;
    ctx.rewrite()?;

    Ok(())
}

pub fn add_page_one(ctx: &mut Context, message: &str) -> Result<()> {
    shell_base(ctx, "Quicknav * Interactive > Add - Location")?;

    if ctx.check {
        write!(
            ctx.tty,
            "{}{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Green),
            message, ctx.goto(5, 5),
            color::Fg(color::Reset),
        )?;
    } else {
        write!(
            ctx.tty,
            "{}{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Red),
            message, ctx.goto(5, 5),
            color::Fg(color::Reset),
        )?;
    }
    ctx.flush()?;

    ctx.column = 5;
    ctx.far_right = 5;
    ctx.page = "add_one".to_owned();
    ctx.rewrite()?;

    Ok(())
}

pub fn add_page_two(ctx: &mut Context, message: &str) -> Result<()> {
    shell_base(ctx, "Quicknav * Interactive > Add - Description")?;

    if ctx.check {
        write!(
            ctx.tty,
            "{}{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Green),
            message, ctx.goto(5, 5),
            color::Fg(color::Reset),
        )?;
    } else {
        write!(
            ctx.tty,
            "{}{}{}{}{}",
            ctx.goto(1, 3),
            color::Fg(color::Red),
            message, ctx.goto(5, 5),
            color::Fg(color::Reset),
        )?;
    }
    ctx.flush()?;

    ctx.column = 5;
    ctx.far_right = 5;
    ctx.page = "add_two".to_owned();
    ctx.rewrite()?;


    Ok(())
}

pub fn edit_page_base(ctx: &mut Context) -> Result<()> {
    Ok(())
}

pub fn remove_page_base(ctx: &mut Context) -> Result<()> {
    Ok(())
}
