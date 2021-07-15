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

    /// Buffer containing Vector of chars the user has entered on the current line
    pub content: Vec<char>,

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
            line: 4,
            column: 2,
            far_right: 5,
            content: vec![],
            check: false,
            size: termion::terminal_size().unwrap(),
            shell: AlternateScreen::from(io::stdout().into_raw_mode().unwrap()),
        }
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
        if let true = self.column > 5 {
            self.column -= 1;
            self.goto_ext(self.column, self.line)?;
        }

        Ok(())
    }

    /// Moves the cursor to the right
    pub fn right(&mut self) -> Result<()> {
        // Stops from moving right off the edge of terminal
        if let true = self.column < self.far_right {
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
        // Handle case where we are writing in the middle of the line
        if self.column < self.far_right {
            for (i , _) in self.content.clone().iter().enumerate() {
                if (i + 5) as u16 == self.column {
                    self.content.insert(i, c);
                    self.far_right += 1;

                    if let true = self.column != 5 {
                        self.column += 1
                    }

                    self.rewrite()?;
                    break;
                }
            }

        } else {
            self.push(c);
            write!(self.tty, "{}", c)?;
            self.flush()?
        }

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
        self.far_right = 5;

        write!(self.tty, "{}", self.goto(self.column, self.line),)?;

        self.flush()?;

        Ok(())
    }

    /// Rewrites the existing line
    pub fn rewrite(&mut self) -> Result<()> {
        write!(self.tty, "{}{}",
            clear::CurrentLine, self.goto(1, self.line)
        )?;
        self.flush()?;

        let content: String = self.content.iter().collect();
        write!(self.tty, " >> {}{}", content, self.goto(self.column, self.line))?;
        self.flush()?;

        Ok(())
    }

    /// Empties the buffer containing the current lines content
    pub fn purge(&mut self) {
        self.content = vec![];
    }

    /// Deletes the character following the cursor from the TTY
    pub fn del(&mut self) -> Result<()> {
        // Only delete if we aren't at the rightmost character boundary
        if self.column < self.far_right {
            for (i , _) in self.content.clone().iter().enumerate() {
                if (i + 5) as u16 == self.column {
                    self.content.remove(i);
                    self.far_right -= 1;
                    self.rewrite()?;
                    break;
                }
            }
        }

        Ok(())
    }

    /// Deletes the character preceding the cursor from the TTY
    pub fn backspace(&mut self) -> Result<()> {
        match self.column {
            // Prevents going off the left side of the terminal
            1 | 2 | 3 | 4 | 5 => {}
            _ => {
                if self.column < self.far_right {
                    for (i , _) in self.content.clone().iter().enumerate() {
                        if (i + 6) as u16 == self.column {
                            self.content.remove(i);
                            self.column -= 1;
                            self.far_right -= 1;
                            self.rewrite()?;
                            break;
                        }
                    }
                } else {
                    self.pop();
                    write!(
                        self.tty,
                        "{}{}{}", self.goto(self.column, self.line),
                        " ", self.goto(self.column, self.line),
                    )?;
                    self.flush()?;
                }
            }
        }

        Ok(())
    }
}
