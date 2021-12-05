use crate::shared_functions::tokenize;
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

pub fn get_bgcolors_from_input(input: &str) -> Vec<BgColor> {
    let tokenized_vec = tokenize(input);
    let mut tmp_vec: String = String::new();
    let mut bgcolor_vec: Vec<BgColor> = Vec::new();
    let mut color = false;
    let mut bgcolor_end = false;
    let mut tok_iter = tokenized_vec.iter().peekable();
    while tok_iter.peek() != None {
        let tok_iter_char = tok_iter.next().unwrap().as_str();
        if bgcolor_end {
            let ret_color: BgColor = match tmp_vec.as_str() {
                "BLACK" => BgColor::Black,
                "RED" => BgColor::Red,
                "GREEN" => BgColor::Green,
                "YELLOW" => BgColor::Yellow,
                "BLUE" => BgColor::Blue,
                "MAGENTA" => BgColor::Magenta,
                "CYAN" => BgColor::Cyan,
                "WHITE" => BgColor::White,
                _ => BgColor::Default,
            };
            bgcolor_vec.push(ret_color);
            tmp_vec.clear();
            color = false;
            bgcolor_end = false;
            continue;
        }
        if tok_iter_char == "B" && tok_iter.peek().unwrap().as_str() == "<" {
            color = true;
        } else if color {
            if tok_iter_char == "<" {
                continue;
            } else if tok_iter_char != ">" {
                tmp_vec.push_str(tok_iter_char);
            } else if tok_iter_char == ">" {
                bgcolor_end = true;
                continue;
            }
        }
    }
    bgcolor_vec
}

pub fn get_fgcolors_from_input(input: &str) -> Vec<FgColor> {
    let tokenized_vec = tokenize(input);
    let mut tmp_vec: String = String::new();
    let mut fgcolor_vec: Vec<FgColor> = Vec::new();
    let mut color = false;
    let mut fgcolor_end = false;
    let mut tok_iter = tokenized_vec.iter().peekable();
    while tok_iter.peek() != None {
        let tok_iter_char = tok_iter.next().unwrap().as_str();
        if fgcolor_end {
            let ret_color: FgColor = match tmp_vec.as_str() {
                "BLACK" => FgColor::Black,
                "RED" => FgColor::Red,
                "GREEN" => FgColor::Green,
                "YELLOW" => FgColor::Yellow,
                "BLUE" => FgColor::Blue,
                "MAGENTA" => FgColor::Magenta,
                "CYAN" => FgColor::Cyan,
                "WHITE" => FgColor::White,
                _ => FgColor::Default,
            };
            fgcolor_vec.push(ret_color);
            tmp_vec.clear();
            color = false;
            fgcolor_end = false;
            continue;
        }
        if tok_iter_char == "F" && tok_iter.peek().unwrap().as_str() == "<" {
            color = true;
        } else if color {
            if tok_iter_char == "<" {
                continue;
            } else if tok_iter_char != ">" {
                tmp_vec.push_str(tok_iter_char);
            } else if tok_iter_char == ">" {
                fgcolor_end = true;
                continue;
            }
        }
    }
    fgcolor_vec
}

