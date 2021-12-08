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
    pub fn to_str(self) -> String {
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
    pub fn to_u8(self) -> u8 {
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
    pub fn to_str(self) -> String {
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
    pub fn to_u8(self) -> u8 {
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

pub fn parse_prompt_colors(input: &str) -> String {
    let tokenized_vec = tokenize(input);
    let mut tok_iter = tokenized_vec.iter().peekable();
    let mut es_builder = EscapeSequence::builder();
    let mut es_seqs: Vec<(String, String)> = Vec::new();
    let mut fin_prompt = String::new();
    let mut tmp_string = String::new();
    let mut pos_option = false;
    let mut pos_bgcol = false;
    let mut bgcol = false;
    let mut fgcol = false;
    let mut pos_fgcol = false;
    let mut option = false;
    let mut option_fin = false;
    let mut es_fin = false;
    let mut not_pos_option = true;
    // Go through every character in the input, until end is reached
    while tok_iter.peek() != None {
        // get next char
        // unwrap is safe, won't be none, checking that in the while condition
        let cur_char = tok_iter.next().unwrap().as_str();
        // match for certain key chars like % for options, F for fgcolor, B for bgcolor
        match cur_char {
            "%" => {
                not_pos_option = false;
                option_fin = false;
                pos_option = true;
            }
            "B" => {
                not_pos_option = false;
                option_fin = false;
                pos_bgcol = true;
            }
            "F" => {
                not_pos_option = false;
                option_fin = false;
                pos_fgcol = true;
            }
            _ => {
                // TODO(zeno): Push every char to the prompt and pop it, if it belongs to an option
                // if the char wasn't matched, it is not a possbiel option, but could be an
                // argument for an option, like a color or font effect
                // this is checked later 
                not_pos_option = true;
                // if the option is finished, the escape sequence is finished too, except the char
                // is matched by a possible identifier
                if option_fin {
                    es_fin = true;
                }
            }
        }

        // if an escape sequence has finished, build the sequence and push it in the prompt string
        if es_fin {
            // do nothing if there is nothing in the es_seqs vector, because then we dont have
            // anything to build
            if es_seqs.is_empty() {
                continue;
            }
            // take the identifier and the sequence out of the vector
            // there is an identifier, because we can't know if we ment a fgcol or bgcol just from
            // the color, so I (zeno) introduced an indentifier
            // could be usefull, if on value can mean different things in different contexts
            for (ty, seq) in &es_seqs {
                // default arg, shoudl be reset
                let mut arg = 0;
                // actually check for the identifier, to know, what escape sequence we should use
                match ty.as_str() {
                    "O" => {
                        arg = match seq.as_str() {
                            "i" => FontEffects::Italics.to_u8(),
                            "u" => FontEffects::Underline.to_u8(),
                            _ => 0,
                        }
                    }
                    "B" => {
                        let tmp_arg = match seq.as_str() {
                            "BLACK" => Color::Bg(BgColor::Black),
                            "RED" => Color::Bg(BgColor::Red),
                            "GREEN" => Color::Bg(BgColor::Green),
                            "YELLOW" => Color::Bg(BgColor::Yellow),
                            "BLUE" => Color::Bg(BgColor::Blue),
                            "MAGENTA" => Color::Bg(BgColor::Magenta),
                            "CYAN" => Color::Bg(BgColor::Cyan),
                            "WHITE" => Color::Bg(BgColor::White),
                            _ => Color::Bg(BgColor::White),
                        };
                        arg = match tmp_arg {
                            Color::Fg(fg) => {
                                fg.to_u8()
                            },
                            Color::Bg(bg) => {
                                bg.to_u8()
                            }
                        };
                    }
                    "F" => {
                        let tmp_arg = match seq.as_str() {
                            "BLACK" => Color::Fg(FgColor::Black),
                            "RED" => Color::Fg(FgColor::Red),
                            "GREEN" => Color::Fg(FgColor::Green),
                            "YELLOW" => Color::Fg(FgColor::Yellow),
                            "BLUE" => Color::Fg(FgColor::Blue),
                            "MAGENTA" => Color::Fg(FgColor::Magenta),
                            "CYAN" => Color::Fg(FgColor::Cyan),
                            "WHITE" => Color::Fg(FgColor::White),
                            _ => Color::Fg(FgColor::White),
                        };
                        arg = match tmp_arg {
                            Color::Fg(fg) => {
                                fg.to_u8()
                            },
                            Color::Bg(bg) => {
                                bg.to_u8()
                            }
                        };
                    }
                    _ => (),
                }
                es_builder.append(arg);
            }
            // push finished escape sequence to prompt string
            fin_prompt.push_str(es_builder.build().escape_sequence.as_str());
            // clear the EsBuilder, so the old escape sequence doenst get expended
            es_builder = EsBuilder::new();
            // clear the es_seqs vector from any escape sequences, because we begin a new set
            es_seqs.clear();
            option_fin = false;
            es_fin = false;
        }

        // check if prev determined possible option, color is actually an option, color
        // and set appropriate flags for checking
        if pos_option && cur_char == "{" {
            option = true;
            option_fin = false;
            pos_option = false;
            continue;
        } else if pos_bgcol && cur_char == "<" {
            bgcol = true;
            option_fin = false;
            pos_bgcol= false;
            continue;
        } else if pos_fgcol && cur_char == "<" {
            fgcol = true;
            option_fin = false;
            pos_fgcol = false;
            continue;
        }

        // handle found option
        if option {
            if cur_char != "}" {
                // if option doesn't end, push char to tmp_string
                tmp_string.push_str(cur_char);
            } else if cur_char == "}" {
                // if option ends, push option to es_seqs vec
                option_fin = true;
                es_seqs.push(("O".to_string(), tmp_string.to_owned()));
                tmp_string.clear();
                option = false;
            }
            continue;
        // Handle foudn bgcol
        } else if bgcol {
            if cur_char != ">" {
                // if bgcol doesn't end, push char to tmp_string
                tmp_string.push_str(cur_char);
            } else if cur_char == ">" {
                // if bgcol ends, push option to es_seqs vec
                option_fin = true;
                es_seqs.push(("B".to_string(), tmp_string.to_owned()));
                tmp_string.clear();
                bgcol = false;
            }
            continue;
        // handle found fgcol
        } else if fgcol {
            if cur_char != ">" {
                // if fgcol doesn't end, push char to tmp_string
                tmp_string.push_str(cur_char);
            } else if cur_char == ">" {
                // if fgcol ends, push option to es_seqs vec
                option_fin = true;
                es_seqs.push(("F".to_string(), tmp_string.to_owned()));
                tmp_string.clear();
                fgcol = false;
            }
            continue;
        }

        // check if char belongs to an option, otherwise push it as is to prompt string
        if not_pos_option && (!option || !bgcol || !fgcol) {
            fin_prompt.push_str(cur_char);
        }
    };
    fin_prompt
}
