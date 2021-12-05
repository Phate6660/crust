use std::fmt::{Formatter, Display};

#[derive(Debug, Copy, Clone)]
/// An enum for background colors.
pub enum BgColor {
    Black = 40,
    Red = 41,
    Green = 42,
    Yellow = 43,
    Blue = 44,
    Magenta = 45,
    Cyan = 46,
    White = 47,
    Default = 49,
}

#[derive(Debug, Copy, Clone)]
pub enum FgColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    Default = 39,
}

impl Display for FgColor {
    fn fmt(self: &FgColor, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\x1b[{}m", &self.to_u8())
    }

}

impl Display for BgColor {
    fn fmt(self: &BgColor, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\x1b[{}m", &self.to_u8())
    }

}

// TODO: Support both bg and fg colors in the same impl.
impl FgColor {
    pub fn to_u8(self: &FgColor) -> u8 {
        match self {
            FgColor::Black => 30,
            FgColor::Red => 31,
            FgColor::Green => 32,
            FgColor::Yellow => 33,
            FgColor::Blue => 34,
            FgColor::Magenta => 35,
            FgColor::Cyan => 36,
            FgColor::White => 37,
            FgColor::Default => 39,
        }
    }

    pub fn to_str(self: &FgColor) -> &'static str {
        match self {
            FgColor::Black => "BLACK",
            FgColor::Red => "RED",
            FgColor::Green => "GREEN",
            FgColor::Yellow => "YELLOW",
            FgColor::Blue => "BLUE",
            FgColor::Magenta => "MAGENTA",
            FgColor::Cyan => "CYAN",
            FgColor::White => "WHITE",
            FgColor::Default => "DEFAULT",
        }
    }
}

// TODO: Support both bg and fg colors in the same impl.
impl BgColor {
    pub fn to_u8(self: &BgColor) -> u8 {
        match self {
            BgColor::Black => 40,
            BgColor::Red => 41,
            BgColor::Green => 42,
            BgColor::Yellow => 43,
            BgColor::Blue => 44,
            BgColor::Magenta => 45,
            BgColor::Cyan => 46,
            BgColor::White => 47,
            BgColor::Default => 49,
        }
    }

    pub fn to_str(self: &BgColor) -> &'static str {
        match self {
            BgColor::Black => "BLACK",
            BgColor::Red => "RED",
            BgColor::Green => "GREEN",
            BgColor::Yellow => "YELLOW",
            BgColor::Blue => "BLUE",
            BgColor::Magenta => "MAGENTA",
            BgColor::Cyan => "CYAN",
            BgColor::White => "WHITE",
            BgColor::Default => "DEFAULT",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EscapeSequence {
    escape_sequence: String,
}

impl EscapeSequence {
    pub fn builder() -> EsBuilder {
        EsBuilder::default()
    }
}

pub struct EsBuilder {
    escape_sequence: String,
}

impl Default for EsBuilder {
    fn default() -> EsBuilder {
        EsBuilder {
            escape_sequence: String::from("\x1b["),
        }
    }
}

impl EsBuilder {
    pub fn new() -> EsBuilder {
        EsBuilder {
            escape_sequence: String::from("\x1b["),
        }
    }

    pub fn append(mut self, argument: u8) -> EsBuilder {
        self.escape_sequence.push(';');
        self.escape_sequence.push_str(&argument.to_string());
        self
    }

    pub fn build(mut self) -> EscapeSequence {
        self.escape_sequence.push('m');
        EscapeSequence {
            escape_sequence: self.escape_sequence,
        }
    }
}

