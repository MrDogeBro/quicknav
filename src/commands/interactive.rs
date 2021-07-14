use anyhow::{anyhow, Result};

use std::fmt;
use std::fs::File;
use std::io::{self, Write};

use termion;
use termion::{get_tty, color, clear, cursor};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::*;


/// Structure representing the context of the shell
struct Context {

    /// The TTY to read from and write to
    tty: File,

    /// The current line number
    line: u16,

    /// The current column number
    column: u16,

    /// The rightmost border based on characters entered on the current line
    far_right: u16,

    /// Buffer containing String of chars the user has entered on the current line
    content: String,

    /// A check for proceeding to the next phase of the interactive shell
    check: bool,

    /// The side of the terminal window
    size: (u16, u16),

    /// The alternate screen we are reading from and writing to
    shell: AlternateScreen<RawTerminal<io::Stdout>>,
}


#[derive(Debug)]
/// Enum representing the types we will write to the TTY
enum Line {

    /// Represents a char
    Char(char),

    /// Represents a String
    Str(String),
}

impl Line {

    /// Returns the length of the line
    fn len(&self) -> u16 {

        match self {

            Line::Str(s) => s.len() as u16,
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

impl Context {

    /// Constructs a new Context
    fn new(tty: File) -> Self {

        Context {

            tty,
            line: 3,
            column: 1,
            far_right: 1,
            content: String::new(),
            check: false,
            size: termion::terminal_size().unwrap(),
            shell: AlternateScreen::from(
                io::stdout().into_raw_mode().unwrap()
            ),
        }
    }

    /// Initializes the shell
    fn init_shell(&mut self) -> Result<()> {

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

        write!(self.shell, "{}What would you like to do?", self.goto(1, 3),)?;
        self.flush()?;
        self.new_line()?;
        self.new_line()?;

        write!(self.shell, "{} [1] {}Add", color::Fg(color::Green), color::Fg(color::Reset))?;
        self.flush()?;
        self.new_line()?;

        write!(self.shell, "{} [2] {}Edit", color::Fg(color::Yellow), color::Fg(color::Reset))?;
        self.flush()?;
        self.new_line()?;

        write!(self.shell, "{} [3] {}Remove", color::Fg(color::Red), color::Fg(color::Reset))?;
        self.flush()?;
        self.new_line()?;
        self.new_line()?;

        write!(self.shell, " >> ")?;
        self.flush()?;

        Ok(())
    }

    /// Moves the cursor to the given location when used inside write! macro
    fn goto(&self, column: u16, line: u16) -> cursor::Goto {

        cursor::Goto(column, line)
    }

    /// Moves the cursor to the given location without a write! macro
    fn goto_ext(&mut self, column: u16, line: u16) -> Result<()> {

        write!(self.tty, "{}", self.goto(column, line))?;
        self.flush()?;
        Ok(())
    }

    /// Moves the cursor to the left
    fn left(&mut self) -> Result<()> {

        if let false = self.column == 1 {

            self.column -= 1;
            self.goto_ext(self.column, self.line)?;
        }

        Ok(())
    }

    /// Moves the cursor to the right
    fn right(&mut self) -> Result<()> {

        if let true = self.column < self.far_right {

            self.column += 1;
            self.goto_ext(self.column, self.line)?;
        }

        Ok(())
    }

    /// Pushes a char onto the buffer storing the current lines content
    fn push(&mut self, c: char) {

        self.column += 1;
        self.far_right += 1;
        self.content.push(c);
    }

    /// Pops the last char from the buffer storing the current lines content
    fn pop(&mut self) {

        self.column -= 1;
        self.far_right -= 1;
        self.content.pop();
    }

    /// Flushes the shell
    fn flush(&mut self) -> Result<()> {

        self.shell.flush()?;

        Ok(())
    }

    /// Writes a char to the TTY
    fn write_char(&mut self, c: char) -> Result<()> {

        self.push(c);
        write!(self.tty, "{}", c)?;
        self.flush()?;

        Ok(())
    }

    /// Writes a line to the TTY
    fn write_line(&mut self, line: Line) -> Result<()> {

        write!(self.tty, "{}", line)?;
        self.flush()?;

        Ok(())
    }

    /// Write a new line to the TTY
    fn new_line(&mut self) -> Result<()> {

        // TODO -- use a process input function to parse user input as they hit enter

        // Position the cursor one line down, in the first column
        self.column = 1;
        self.line += 1;
        self.far_right = 1;

        write!(
            self.tty, "{}",
            self.goto(self.column, self.line),
        )?;

        self.flush()?;

        Ok(())
    }

    /// Empties the buffer containing the current lines content
    fn purge(&mut self) {

        self.content = String::new();
    }

    /// Deletes the character following the cursor from the TTY
    fn del(&mut self) -> Result<()> {

        Ok(())
    }

    /// Deletes the character preceding the cursor from the TTY
    fn backspace(&mut self) -> Result<()> {

        match self.column {

            // Prevents going off the left side of the terminal
            1 => {},
            _ => {

                self.pop();

                write!(self.tty,
                    "{}{}{}", self.goto(self.column, self.line),
                    " ", self.goto(self.column, self.line),
                )?;

                self.flush()?;
            }
        }

        Ok(())
    }
}

/// Public facing entry point to the interactive shell
pub fn shell(command: Option<String>) -> Result<i32> {

    interactive()
}

fn interactive() -> Result<i32> {

    // Initialize a new shell and Context
    let stdin = io::stdin();
    let mut ctx = Context::new(get_tty()?);
    ctx.init_shell()?;

    // Iterate over key presses and act accordingly
    for c in stdin.keys().map(|c| c.unwrap()) {

        match c {

            Key::Ctrl('c') => break,
            Key::Backspace => ctx.backspace()?,
            Key::Delete => ctx.del()?,
            Key::Up => {}
            Key::Down => {}
            Key::Left => ctx.left()?,
            Key::Right => ctx.right()?,
            Key::Char('\n') => {
                ctx.new_line()?;
                ctx.write_line(Line::Str(format!("Content: {}", ctx.content.clone())))?; // just testing this here
                ctx.new_line()?;
                ctx.purge();
            }
            Key::Char('0') => ctx.write_char('0')?,
            Key::Char('1') => ctx.write_char('1')?,
            Key::Char('2') => ctx.write_char('2')?,
            Key::Char('3') => ctx.write_char('3')?,
            Key::Char('4') => ctx.write_char('4')?,
            Key::Char('5') => ctx.write_char('5')?,
            Key::Char('6') => ctx.write_char('6')?,
            Key::Char('7') => ctx.write_char('7')?,
            Key::Char('8') => ctx.write_char('8')?,
            Key::Char('9') => ctx.write_char('9')?,
            Key::Char('a') => ctx.write_char('a')?,
            Key::Char('b') => ctx.write_char('b')?,
            Key::Char('c') => ctx.write_char('c')?,
            Key::Char('d') => ctx.write_char('d')?,
            Key::Char('e') => ctx.write_char('e')?,
            Key::Char('f') => ctx.write_char('f')?,
            Key::Char('g') => ctx.write_char('g')?,
            Key::Char('h') => ctx.write_char('h')?,
            Key::Char('i') => ctx.write_char('i')?,
            Key::Char('j') => ctx.write_char('j')?,
            Key::Char('k') => ctx.write_char('k')?,
            Key::Char('l') => ctx.write_char('l')?,
            Key::Char('m') => ctx.write_char('m')?,
            Key::Char('n') => ctx.write_char('n')?,
            Key::Char('o') => ctx.write_char('o')?,
            Key::Char('p') => ctx.write_char('p')?,
            Key::Char('q') => ctx.write_char('q')?,
            Key::Char('r') => ctx.write_char('r')?,
            Key::Char('s') => ctx.write_char('s')?,
            Key::Char('t') => ctx.write_char('t')?,
            Key::Char('u') => ctx.write_char('u')?,
            Key::Char('v') => ctx.write_char('v')?,
            Key::Char('w') => ctx.write_char('w')?,
            Key::Char('x') => ctx.write_char('x')?,
            Key::Char('y') => ctx.write_char('y')?,
            Key::Char('z') => ctx.write_char('z')?,
            Key::Char('A') => ctx.write_char('A')?,
            Key::Char('B') => ctx.write_char('B')?,
            Key::Char('C') => ctx.write_char('C')?,
            Key::Char('D') => ctx.write_char('D')?,
            Key::Char('E') => ctx.write_char('E')?,
            Key::Char('F') => ctx.write_char('F')?,
            Key::Char('G') => ctx.write_char('G')?,
            Key::Char('H') => ctx.write_char('H')?,
            Key::Char('I') => ctx.write_char('I')?,
            Key::Char('J') => ctx.write_char('J')?,
            Key::Char('K') => ctx.write_char('K')?,
            Key::Char('L') => ctx.write_char('L')?,
            Key::Char('M') => ctx.write_char('M')?,
            Key::Char('N') => ctx.write_char('N')?,
            Key::Char('O') => ctx.write_char('O')?,
            Key::Char('P') => ctx.write_char('P')?,
            Key::Char('Q') => ctx.write_char('Q')?,
            Key::Char('R') => ctx.write_char('R')?,
            Key::Char('S') => ctx.write_char('S')?,
            Key::Char('T') => ctx.write_char('T')?,
            Key::Char('U') => ctx.write_char('U')?,
            Key::Char('V') => ctx.write_char('V')?,
            Key::Char('W') => ctx.write_char('W')?,
            Key::Char('X') => ctx.write_char('X')?,
            Key::Char('Y') => ctx.write_char('Y')?,
            Key::Char('Z') => ctx.write_char('Z')?,
            Key::Char('!') => ctx.write_char('!')?,
            Key::Char('"') => ctx.write_char('"')?,
            Key::Char('#') => ctx.write_char('#')?,
            Key::Char('$') => ctx.write_char('$')?,
            Key::Char('%') => ctx.write_char('%')?,
            Key::Char('&') => ctx.write_char('&')?,
            Key::Char('(') => ctx.write_char('(')?,
            Key::Char(')') => ctx.write_char(')')?,
            Key::Char('*') => ctx.write_char('*')?,
            Key::Char('+') => ctx.write_char('+')?,
            Key::Char(',') => ctx.write_char(',')?,
            Key::Char('-') => ctx.write_char('-')?,
            Key::Char('.') => ctx.write_char('.')?,
            Key::Char('/') => ctx.write_char('/')?,
            Key::Char(':') => ctx.write_char(':')?,
            Key::Char(';') => ctx.write_char(';')?,
            Key::Char('<') => ctx.write_char('<')?,
            Key::Char('=') => ctx.write_char('=')?,
            Key::Char('>') => ctx.write_char('>')?,
            Key::Char('?') => ctx.write_char('?')?,
            Key::Char('@') => ctx.write_char('@')?,
            Key::Char('[') => ctx.write_char('[')?,
            Key::Char(']') => ctx.write_char(']')?,
            Key::Char('^') => ctx.write_char('^')?,
            Key::Char('_') => ctx.write_char('_')?,
            Key::Char('`') => ctx.write_char('`')?,
            Key::Char('{') => ctx.write_char('{')?,
            Key::Char('|') => ctx.write_char('|')?,
            Key::Char('}') => ctx.write_char('}')?,
            Key::Char('~') => ctx.write_char('~')?,
            Key::Char(' ') => ctx.write_char(' ')?,
            Key::Char('\'') => ctx.write_char('\'')?,
            Key::Char('\\') => ctx.write_char('\\')?,
            Key::Char('\t') => ctx.write_char('\t')?,
            Key::Char('\r') => ctx.write_char('\r')?,
            _ => {write!(ctx.tty, "bad {:?}", c)?},
         }

        ctx.flush()?;
    }

    // Exit the interactive shell
    write!(ctx.tty, "{}", ToMainScreen)?;
    ctx.flush()?;

    Ok(0)
}
