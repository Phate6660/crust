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
/// An enum for foreground colors.
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

#[derive(Debug, Copy, Clone)]
/// A unified enum for accessing both foreground and background colors.
pub enum Color {
    Bg(BgColor),
    Fg(FgColor),
}

impl Display for BgColor {
    // Displays the full escape sequence.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", &self.to_u8())
    }
}

impl Display for FgColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", &self.to_u8())
    }
}

impl BgColor {
    pub fn to_str(self: &BgColor) -> String {
        match self {
            BgColor::Black => "BLACK".to_string(),
            BgColor::Red => "RED".to_string(),
            BgColor::Green => "GREEN".to_string(),
            BgColor::Yellow => "YELLOW".to_string(),
            BgColor::Blue => "BLUE".to_string(),
            BgColor::Magenta => "MAGENTA".to_string(),
            BgColor::Cyan => "CYAN".to_string(),
            BgColor::White => "WHITE".to_string(),
            BgColor::Default => "DEFAULT".to_string(),
        }
    }
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
}

impl FgColor {
    pub fn to_str(self: &FgColor) -> String {
        match self {
            FgColor::Black => "BLACK".to_string(),
            FgColor::Red => "RED".to_string(),
            FgColor::Green => "GREEN".to_string(),
            FgColor::Yellow => "YELLOW".to_string(),
            FgColor::Blue => "BLUE".to_string(),
            FgColor::Magenta => "MAGENTA".to_string(),
            FgColor::Cyan => "CYAN".to_string(),
            FgColor::White => "WHITE".to_string(),
            FgColor::Default => "DEFAULT".to_string(),
        }
    }
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
}

pub enum FontEffects {
    Italics = 3,
    Underline = 4,
}

impl Display for FontEffects {
    // Displays the full escape sequence.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", &self.to_u8())
    }
}

impl FontEffects {
    pub fn to_str(self: &FontEffects) -> String {
        match self {
            FontEffects::Italics => "3".to_string(),
            FontEffects::Underline => "4".to_string(),
        }
    }
    pub fn to_u8(self: &FontEffects) -> u8 {
        match self {
            FontEffects::Italics => 3,
            FontEffects::Underline => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EscapeSequence {
    escape_sequence: String,
}

impl Display for EscapeSequence {
    // Displays the full escape sequence.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.escape_sequence)
    }
}

impl EscapeSequence {
    pub fn builder() -> EsBuilder {
        EsBuilder::default()
    }
}

#[derive(Clone, Debug)]
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

    pub fn append(&mut self, argument: u8) {
        self.escape_sequence.push_str(&argument.to_string());
        self.escape_sequence.push(';');
    }

    pub fn build(&mut self) -> EscapeSequence {
        self.escape_sequence.pop();
        self.escape_sequence.push('m');
        EscapeSequence {
            escape_sequence: self.escape_sequence.clone(),
        }
    }
}

pub fn get_colors_from_input(input: &str) -> Vec<Color> {
    let tokenized_vec = tokenize(input);
    let mut tmp_vec: String = String::new();
    let mut color_vec: Vec<Color> = Vec::new();
    let mut color = false;
    let mut bg_color = false;
    let mut fg_color = false;
    let mut tok_iter = tokenized_vec.iter().peekable();
    while tok_iter.peek() != None {
        let tok_iter_char = tok_iter.next().unwrap().as_str();
        if tok_iter_char == "B" && tok_iter.peek().unwrap().as_str() == "<" {
            bg_color = true;
            color = true;
            continue;
        } else if tok_iter_char == "F" && tok_iter.peek().unwrap().as_str() == "<" {
            fg_color = true;
            color = true;
            continue;
        } else if color {
            if tok_iter_char == "<" {
                continue;
            } else if tok_iter_char != ">" {
                tmp_vec.push_str(tok_iter_char);
                continue;
            } else if tok_iter_char == ">" {
                if fg_color {
                    let ret_color: Color = match tmp_vec.as_str() {
                        "BLACK" => Color::Fg(FgColor::Black),
                        "RED" => Color::Fg(FgColor::Red),
                        "GREEN" => Color::Fg(FgColor::Green),
                        "YELLOW" => Color::Fg(FgColor::Yellow),
                        "BLUE" => Color::Fg(FgColor::Blue),
                        "MAGENTA" => Color::Fg(FgColor::Magenta),
                        "CYAN" => Color::Fg(FgColor::Cyan),
                        "WHITE" => Color::Fg(FgColor::White),
                        _ => Color::Fg(FgColor::Default),
                    };
                    color_vec.push(ret_color);
                    tmp_vec.clear();
                    color = false;
                    fg_color = false;
                    continue;
                } else if bg_color {
                    let ret_color: Color = match tmp_vec.as_str() {
                        "BLACK" => Color::Bg(BgColor::Black),
                        "RED" => Color::Bg(BgColor::Red),
                        "GREEN" => Color::Bg(BgColor::Green),
                        "YELLOW" => Color::Bg(BgColor::Yellow),
                        "BLUE" => Color::Bg(BgColor::Blue),
                        "MAGENTA" => Color::Bg(BgColor::Magenta),
                        "CYAN" => Color::Bg(BgColor::Cyan),
                        "WHITE" => Color::Bg(BgColor::White),
                        _ => Color::Bg(BgColor::Default),
                    };
                    color_vec.push(ret_color);
                    tmp_vec.clear();
                    color = false;
                    bg_color = false;
                    continue;
                } else {
                    // Do nothing.
                }
                continue;
            }
        } else {
            // Do nothing.
        }
    }
    color_vec
}
