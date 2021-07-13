use anyhow::{anyhow, Result};

use std::fs::File;
use std::io::{self, BufRead, Read, Stdin, Stdout, Write};
use std::collections::HashMap;
use colored::*;

use termion;
use termion::{get_tty, color, clear, cursor};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::*;

struct Context {
    tty: File,
    line: u16,
    column: u16,
    content: String,
    check: bool,
    size: (u16, u16),
    shell: AlternateScreen<RawTerminal<Stdout>>,
    stdin: Stdin,
}

impl Context {
    fn new(tty: File) -> Self {
        Context {
            tty,
            line: 3,
            column: 1,
            content: String::new(),
            check: false,
            size: termion::terminal_size().unwrap(),
            stdin: io::stdin(),
            shell: AlternateScreen::from(
                io::stdout().into_raw_mode().unwrap()
            )
        }
    }

    fn write<T>(&self, string: &str, args: &[T]) -> Result<i32> {
        // TODO
        Ok(0)
    }
}

pub fn shell(command: Option<String>) -> Result<i32> {
    interactive_shell(command)
}

fn init_shell(ctx: &mut Context) {
    write!(
        ctx.tty,
        "{}{}{}Welcome to the quicknav interactive shell.{}",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        color::Fg(color::Reset),
    ).unwrap();

    write!(
        ctx.tty,
        "{}{} Ctrl+c{} {}to exit{}{}{}",
        cursor::Goto(ctx.size.0 + 5, ctx.size.1 - 2),
        termion::style::Bold,
        termion::style::Reset,
        color::Fg(color::Red),
        termion::style::Reset,
        color::Fg(color::Reset),
        cursor::Goto(1, 3),
    ).unwrap();

    ctx.tty.flush().unwrap();
}

fn interactive_shell(command: Option<String>) -> Result<i32> {
    let mut ctx = Context::new(get_tty()?);
    init_shell(&mut ctx);

    for c in ctx.stdin.keys().map(|c| c.unwrap()) {
        ctx.column += 1;

        match c {
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                ctx.column = 1;
                ctx.line += 1;
                write!(ctx.tty, "{}", cursor::Goto(ctx.column, ctx.line))?;
            }
            Key::Backspace => {
                match ctx.column {
                    1 | 2 => {},
                    _ => {
                        ctx.column -= 2;
                        write!(ctx.tty, "{}{}", termion::clear::CurrentLine, cursor::Goto(ctx.column, ctx.line))?;
                        ctx.content.pop();
                    }
                }
            }
            Key::Char('0') => write!(ctx.tty, "{}", '0')?,
            Key::Char('1') => write!(ctx.tty, "{}", '1')?,
            Key::Char('2') => write!(ctx.tty, "{}", '2')?,
            Key::Char('3') => write!(ctx.tty, "{}", '3')?,
            Key::Char('4') => write!(ctx.tty, "{}", '4')?,
            Key::Char('5') => write!(ctx.tty, "{}", '5')?,
            Key::Char('6') => write!(ctx.tty, "{}", '6')?,
            Key::Char('7') => write!(ctx.tty, "{}", '7')?,
            Key::Char('8') => write!(ctx.tty, "{}", '8')?,
            Key::Char('9') => write!(ctx.tty, "{}", '9')?,
            Key::Char('a') => write!(ctx.tty, "{}", 'a')?,
            Key::Char('b') => write!(ctx.tty, "{}", 'b')?,
            Key::Char('c') => write!(ctx.tty, "{}", 'c')?,
            Key::Char('d') => write!(ctx.tty, "{}", 'd')?,
            Key::Char('e') => write!(ctx.tty, "{}", 'e')?,
            Key::Char('f') => write!(ctx.tty, "{}", 'f')?,
            Key::Char('g') => write!(ctx.tty, "{}", 'g')?,
            Key::Char('h') => write!(ctx.tty, "{}", 'h')?,
            Key::Char('i') => write!(ctx.tty, "{}", 'i')?,
            Key::Char('j') => write!(ctx.tty, "{}", 'j')?,
            Key::Char('k') => write!(ctx.tty, "{}", 'k')?,
            Key::Char('l') => write!(ctx.tty, "{}", 'l')?,
            Key::Char('m') => write!(ctx.tty, "{}", 'm')?,
            Key::Char('n') => write!(ctx.tty, "{}", 'n')?,
            Key::Char('o') => write!(ctx.tty, "{}", 'o')?,
            Key::Char('p') => write!(ctx.tty, "{}", 'p')?,
            Key::Char('q') => write!(ctx.tty, "{}", 'q')?,
            Key::Char('r') => write!(ctx.tty, "{}", 'r')?,
            Key::Char('s') => write!(ctx.tty, "{}", 's')?,
            Key::Char('t') => write!(ctx.tty, "{}", 't')?,
            Key::Char('u') => write!(ctx.tty, "{}", 'u')?,
            Key::Char('v') => write!(ctx.tty, "{}", 'v')?,
            Key::Char('w') => write!(ctx.tty, "{}", 'w')?,
            Key::Char('x') => write!(ctx.tty, "{}", 'x')?,
            Key::Char('y') => write!(ctx.tty, "{}", 'y')?,
            Key::Char('z') => write!(ctx.tty, "{}", 'z')?,
            Key::Char('A') => write!(ctx.tty, "{}", 'A')?,
            Key::Char('B') => write!(ctx.tty, "{}", 'B')?,
            Key::Char('C') => write!(ctx.tty, "{}", 'C')?,
            Key::Char('D') => write!(ctx.tty, "{}", 'D')?,
            Key::Char('E') => write!(ctx.tty, "{}", 'E')?,
            Key::Char('F') => write!(ctx.tty, "{}", 'F')?,
            Key::Char('G') => write!(ctx.tty, "{}", 'G')?,
            Key::Char('H') => write!(ctx.tty, "{}", 'H')?,
            Key::Char('I') => write!(ctx.tty, "{}", 'I')?,
            Key::Char('J') => write!(ctx.tty, "{}", 'J')?,
            Key::Char('K') => write!(ctx.tty, "{}", 'K')?,
            Key::Char('L') => write!(ctx.tty, "{}", 'L')?,
            Key::Char('M') => write!(ctx.tty, "{}", 'M')?,
            Key::Char('N') => write!(ctx.tty, "{}", 'N')?,
            Key::Char('O') => write!(ctx.tty, "{}", 'O')?,
            Key::Char('P') => write!(ctx.tty, "{}", 'P')?,
            Key::Char('Q') => write!(ctx.tty, "{}", 'Q')?,
            Key::Char('R') => write!(ctx.tty, "{}", 'R')?,
            Key::Char('S') => write!(ctx.tty, "{}", 'S')?,
            Key::Char('T') => write!(ctx.tty, "{}", 'T')?,
            Key::Char('U') => write!(ctx.tty, "{}", 'U')?,
            Key::Char('V') => write!(ctx.tty, "{}", 'V')?,
            Key::Char('W') => write!(ctx.tty, "{}", 'W')?,
            Key::Char('X') => write!(ctx.tty, "{}", 'X')?,
            Key::Char('Y') => write!(ctx.tty, "{}", 'Y')?,
            Key::Char('Z') => write!(ctx.tty, "{}", 'Z')?,
            Key::Char('!') => write!(ctx.tty, "{}", '!')?,
            Key::Char('"') => write!(ctx.tty, "{}", '"')?,
            Key::Char('#') => write!(ctx.tty, "{}", '#')?,
            Key::Char('$') => write!(ctx.tty, "{}", '$')?,
            Key::Char('%') => write!(ctx.tty, "{}", '%')?,
            Key::Char('&') => write!(ctx.tty, "{}", '&')?,
            Key::Char('(') => write!(ctx.tty, "{}", '(')?,
            Key::Char(')') => write!(ctx.tty, "{}", ')')?,
            Key::Char('*') => write!(ctx.tty, "{}", '*')?,
            Key::Char('+') => write!(ctx.tty, "{}", '+')?,
            Key::Char(',') => write!(ctx.tty, "{}", ',')?,
            Key::Char('-') => write!(ctx.tty, "{}", '-')?,
            Key::Char('.') => write!(ctx.tty, "{}", '.')?,
            Key::Char('/') => write!(ctx.tty, "{}", '/')?,
            Key::Char(':') => write!(ctx.tty, "{}", ':')?,
            Key::Char(';') => write!(ctx.tty, "{}", ';')?,
            Key::Char('<') => write!(ctx.tty, "{}", '<')?,
            Key::Char('=') => write!(ctx.tty, "{}", '=')?,
            Key::Char('>') => write!(ctx.tty, "{}", '>')?,
            Key::Char('?') => write!(ctx.tty, "{}", '?')?,
            Key::Char('@') => write!(ctx.tty, "{}", '@')?,
            Key::Char('[') => write!(ctx.tty, "{}", '[')?,
            Key::Char(']') => write!(ctx.tty, "{}", ']')?,
            Key::Char('^') => write!(ctx.tty, "{}", '^')?,
            Key::Char('_') => write!(ctx.tty, "{}", '_')?,
            Key::Char('`') => write!(ctx.tty, "{}", '`')?,
            Key::Char('{') => write!(ctx.tty, "{}", '{')?,
            Key::Char('|') => write!(ctx.tty, "{}", '|')?,
            Key::Char('}') => write!(ctx.tty, "{}", '}')?,
            Key::Char('~') => write!(ctx.tty, "{}", '~')?,
            Key::Char(' ') => write!(ctx.tty, "{}", ' ')?,
            Key::Char('\'') => write!(ctx.tty, "{}", '\'')?,
            Key::Char('\\') => write!(ctx.tty, "{}", '\\')?,
            Key::Char('\t') => write!(ctx.tty, "{}", '\t')?,
            Key::Char('\r') => write!(ctx.tty, "{}", '\r')?,
            _ => {write!(ctx.tty, "bad {:?}", c)?},
         }

        ctx.tty.flush()?;
    }


    write!(ctx.tty, "{}", ToMainScreen)?;
    ctx.tty.flush()?;

    Ok(0)
}
