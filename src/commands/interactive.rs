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

/// Structure representing the context of the shell
struct Context {

    /// The TTY to read from and write to
    tty: File,

    /// The current line number
    line: u16,

    /// The current column number
    column: u16,

    /// Buffer containing String of chars the user has entered on the current line
    content: String,

    /// A check for proceeding to the next phase of the interactive shell
    check: bool,

    /// The side of the terminal window
    size: (u16, u16),

    /// The alternate screen we are reading from and writing to
    shell: AlternateScreen<RawTerminal<io::Stdout>>,
}

impl Context {

    /// Constructs a new Context
    fn new(tty: File) -> Self {

        Context {

            tty,
            line: 3,
            column: 1,
            content: String::new(),
            check: false,
            size: termion::terminal_size().unwrap(),
            shell: AlternateScreen::from(
                io::stdout().into_raw_mode().unwrap()
            ),
        }
    }

    /// Initializes the shell
    fn init_shell(&mut self) -> Result<()>{

        write!(

            self.shell,
            "{}{}{}Welcome to the quicknav interactive shell.{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Green),
            color::Fg(color::Reset),
            cursor::Goto(1, 2),
            "-".repeat(42),
        )?;

        write!(

            self.shell,
            "{}{} Ctrl+c{} {}to exit{}{}{}",
            cursor::Goto(self.size.0 + 2, self.size.1 - 2),
            termion::style::Bold,
            termion::style::Reset,
            color::Fg(color::Red),
            termion::style::Reset,
            color::Fg(color::Reset),
            cursor::Goto(1, 3),
        )?;

        self.shell.flush()?;

        Ok(())
    }

    /// Pushes a char onto the buffer storing the current lines content
    fn push(&mut self, c: char) {

        self.content.push(c);
    }

    /// Pops the last char from the buffer storing the current lines content
    fn pop(&mut self) {

        self.content.pop();
    }

    /// Writes a char to the TTY
    fn write_char(&mut self, c: char) -> Result<()> {

        self.column += 1;
        self.push(c);

        write!(self.tty, "{}", c)?;

        Ok(())
    }

    /// Writes a line to the TTY
    fn write_line(&mut self, line: Line) -> Result<()> {

        write!(self.tty, "{}", line)?;
        self.new_line()?;
        self.shell.flush()?;

        Ok(())
    }

    /// Write a new line to the TTY
    fn new_line(&mut self) -> Result<()> {

        // Position the cursor one line down, in the first column
        self.column = 1;
        self.line += 1;

        write!(
            self.tty, "{}",
            cursor::Goto(self.column, self.line),
        )?;

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

                self.column -= 1;
                self.pop();

                write!(self.tty,
                    "{}{}{}", cursor::Goto(self.column, self.line),
                    " ", cursor::Goto(self.column, self.line),
                )?;
            }
        }

        Ok(())
    }
}

/// Public facing entry point to the interactive shell
pub fn shell(command: Option<String>) -> Result<i32> {

    interactive_shell()
}

fn interactive_shell() -> Result<i32> {

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
            Key::Left => {}
            Key::Right => {}
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

        ctx.shell.flush()?;
    }

    // Exit the interactive shell
    write!(ctx.tty, "{}", ToMainScreen)?;
    ctx.shell.flush()?;

    Ok(0)
}
