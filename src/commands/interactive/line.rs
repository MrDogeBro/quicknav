use std::fmt;

#[derive(Debug)]
/// Enum representing the types we will write to the TTY
pub enum Line {
    /// Represents a char
    Char(char),

    /// Represents a String
    Str(String),
}

impl Line {
    /// Returns the length of the line
    pub fn len(&self) -> u16 {
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
