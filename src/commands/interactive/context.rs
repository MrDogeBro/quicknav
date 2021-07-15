use anyhow::Result;

use std::fs::File;
use std::io::{self, Write};

use termion::color;
use termion::cursor;
use termion::clear;
use termion::screen::AlternateScreen;
use termion::raw::{RawTerminal, IntoRawMode};

use super::Line;

/// Structure representing the context of the shell
pub struct Context {
    /// The TTY to read from and write to
    pub tty: File,

    /// The current line number
    pub line: u16,

    /// The current column number
    pub column: u16,

    /// The rightmost border based on characters entered on the current line
    pub far_right: u16,

    /// Buffer containing String of chars the user has entered on the current line
    pub content: String,

    /// A check for proceeding to the next phase of the interactive shell
    pub check: bool,

    /// The side of the terminal window
    pub size: (u16, u16),

    /// The alternate screen we are reading from and writing to
    pub shell: AlternateScreen<RawTerminal<io::Stdout>>,
}

impl Context {
    /// Constructs a new Context
    pub fn new(tty: File) -> Self {
        Context {
            tty,
            line: 3,
            column: 1,
            far_right: 1,
            content: String::new(),
            check: false,
            size: termion::terminal_size().unwrap(),
            shell: AlternateScreen::from(io::stdout().into_raw_mode().unwrap()),
        }
    }

    /// Initializes the shell
    pub fn base_shell(&mut self) -> Result<()> {
        write!(
            self.shell,
            "{}{}{}Welcome to the quicknav interactive shell.{}{}{}",
            clear::All,
            self.goto(1, 1),
            color::Fg(color::Green),
            color::Fg(color::Reset),
            self.goto(1, 2),
            "-".repeat(42),
        )?;
        self.flush()?;

        write!(
            self.shell,
            "{}{} Ctrl+c{} {}to exit{}{}",
            self.goto(self.size.0 + 2, self.size.1 - 2),
            termion::style::Bold,
            termion::style::Reset,
            color::Fg(color::Red),
            termion::style::Reset,
            color::Fg(color::Reset),
        )?;
        self.flush()?;

        Ok(())
    }

    // The first menu page
    pub fn welcome_page(&mut self) -> Result<()> {
        write!(self.tty, "{}What would you like to do?", self.goto(1, 3),)?;
        self.flush()?;
        self.new_line()?;
        self.new_line()?;

        write!(
            self.tty,
            "{}[1] {}Add a shortcut",
            color::Fg(color::Green),
            color::Fg(color::Reset)
        )?;
        self.flush()?;
        self.new_line()?;

        write!(
            self.tty,
            "{}[2] {}Edit a shortcut",
            color::Fg(color::Yellow),
            color::Fg(color::Reset)
        )?;
        self.flush()?;
        self.new_line()?;

        write!(
            self.tty,
            "{}[3] {}Remove a shortcut",
            color::Fg(color::Red),
            color::Fg(color::Reset)
        )?;
        self.flush()?;
        self.new_line()?;
        self.new_line()?;

        self.goto_ext(self.column, self.line)?;

        Ok(())
    }

    /// Moves the cursor to the given location when used inside write! macro
    pub fn goto(&self, column: u16, line: u16) -> cursor::Goto {
        cursor::Goto(column, line)
    }

    /// Moves the cursor to the given location without a write! macro
    pub fn goto_ext(&mut self, column: u16, line: u16) -> Result<()> {
        write!(self.tty, "{}", self.goto(column, line))?;
        self.flush()?;

        Ok(())
    }

    /// Moves the cursor to the left
    pub fn left(&mut self) -> Result<()> {
        // Stops from moving left past the edge of the terminal
        if let true = self.column > 1 {
            self.column -= 1;
            self.goto_ext(self.column, self.line)?;
        }

        Ok(())
    }

    /// Moves the cursor to the right
    pub fn right(&mut self) -> Result<()> {
        // Stops from moving right off the edge of terminal
        if let true = (self.column < self.far_right && self.column > 4) {
            self.column += 1;
            self.goto_ext(self.column, self.line)?;
        }

        Ok(())
    }

    /// Pushes a char onto the buffer storing the current lines content
    pub fn push(&mut self, c: char) {
        self.column += 1;
        self.far_right += 1;
        self.content.push(c);
    }

    /// Pops the last char from the buffer storing the current lines content
    pub fn pop(&mut self) {
        self.column -= 1;
        self.far_right -= 1;
        self.content.pop();
    }

    /// Flushes the shell
    pub fn flush(&mut self) -> Result<()> {
        self.shell.flush()?;

        Ok(())
    }

    /// Writes a char to the TTY
    pub fn write_char(&mut self, c: char) -> Result<()> {
        self.push(c);
        write!(self.tty, "{}", c)?;
        self.flush()?;

        Ok(())
    }

    /// Writes a line to the TTY
    pub fn write_line(&mut self, line: Line) -> Result<()> {
        self.column += line.len();
        write!(self.tty, "{}", line)?;
        self.flush()?;

        Ok(())
    }

    /// Write a new line to the TTY
    pub fn new_line(&mut self) -> Result<()> {
        // TODO -- use a process input function to parse user input as they hit enter

        // Position the cursor one line down, in the first column
        self.column = 1;
        self.line += 1;
        self.far_right = 1;

        write!(self.tty, "{}", self.goto(self.column, self.line),)?;

        self.flush()?;

        Ok(())
    }

    /// Empties the buffer containing the current lines content
    pub fn purge(&mut self) {
        self.content = String::new();
    }

    /// Deletes the character following the cursor from the TTY
    pub fn del(&mut self) -> Result<()> {
        Ok(())
    }

    /// Deletes the character preceding the cursor from the TTY
    pub fn backspace(&mut self) -> Result<()> {
        match self.column {
            // Prevents going off the left side of the terminal
            1 => {}
            _ => {
                self.pop();

                write!(
                    self.tty,
                    "{}{}{}",
                    self.goto(self.column, self.line),
                    " ",
                    self.goto(self.column, self.line),
                )?;

                self.flush()?;
            }
        }

        Ok(())
    }
}
