use crate::commands::{cmd, piped_cmd, return_shellcommand, PipedShellCommand, Redirection, ShellCommand};
use crate::prompt::Color;
use sflib::ensure_directory;
use std::env::var as env_var;
use std::path::PathBuf;

#[cfg(feature = "readline")]
use std::process::exit;

#[cfg(feature = "readline")]
use rustyline::{error::ReadlineError, Editor};

#[cfg(not(feature = "readline"))]
use std::io::Write;

/// Holds all important informations for and about the shell.
pub struct ShellState {
    pub args: Vec<String>,
    pub prompt: String,
    pub user: String,
    pub home: String,
    pub na: String,
    pub share_dir: String,
    pub cd_prev_dir: Option<PathBuf>,
}

/// Gets the current time with the format specified if the `time` feature is enabled.
/// Otherwise it returns the format string back.
fn get_time(format: &str) -> String {
    #[cfg(not(feature = "time"))]
    {
        format.to_string()
    }

    #[cfg(feature = "time")]
    {
        let date = chrono::Local::now();
        date.format(format).to_string()
    }
}

// Process the input to run the appropriate builtin or external command.
pub fn process_input(shell_state: &mut ShellState, input: &str) {
    if input.is_empty() {
        return;
    }
    let command = ShellCommand::new(input);
    ShellCommand::run(shell_state, command);
}

#[cfg(feature = "readline")]
pub fn run_loop(prompt: &str, rl: &mut Editor<()>, history_file: &str, mut shell_state: ShellState) {
    loop {
        let prompt = rl.readline(prompt);
        match prompt {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.starts_with("exit") {
                    if line.contains(' ') {
                        let input = line.split(' ').collect::<Vec<&str>>()[1];
                        rl.save_history(&history_file).unwrap();
                        exit(input.parse::<i32>().unwrap_or(0));
                    } else {
                        rl.save_history(&history_file).unwrap();
                        exit(0);
                    }
                }
                process_input(&mut shell_state, &line);
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&history_file).unwrap();
}

#[cfg(not(feature = "readline"))]
pub fn run_loop(prompt: &str, mut shell_state: ShellState) {
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let input = crate::parse_input("interactive");
        process_input(&mut shell_state, &input);
    }
}

// TODO: Unify this with the `lex_tokenized_input` function at line 237.
/// Parses the input and returns a vector `ShellCommand`s.
fn get_commands_from_input(input: &str) -> Vec<ShellCommand> {
    let tokenized_vec = tokenize(input);
    let mut tmp_vec: String = String::new();
    let mut command_vec: Vec<ShellCommand> = Vec::new();
    let mut command = false;
    let mut command_end = false;
    let mut tok_iter = tokenized_vec.iter().peekable();
    while tok_iter.peek() != None {
        let tok_iter_char = tok_iter.next().unwrap().as_str();
        if command_end {
            command_vec.push(ShellCommand::new(tmp_vec.as_str()));
            tmp_vec.clear();
            command = false;
            command_end = false;
            continue;
        }
        if tok_iter_char == "%" && tok_iter.peek().unwrap().as_str() == "(" {
            command = true;
        } else if command {
            if tok_iter_char == "(" {
                continue;
            } else if tok_iter_char != ")" {
                tmp_vec.push_str(tok_iter_char);
            } else if tok_iter_char == ")" {
                command_end = true;
                continue;
            }
        }
    }
    command_vec
}

impl ShellState {
    /// Initalizes the shell state with all the informations needed.
    ///
    /// `cd_prev_dir` doesnt hold a value, because there is no previous dir yet.
    pub fn init() -> ShellState {
        let args = std::env::args().collect();
        let prompt = env_var("PROMPT").unwrap_or_else(|_| String::from("F<BLACK>B<CYAN>%{i}%{u}[crust]:%{re} "));
        println!("{}", prompt);
        let user_command = return_shellcommand(String::from("whoami"), Vec::new(), Redirection::NoOp);
        let user = env_var("USER").unwrap_or_else(|_| cmd(&user_command)).trim().to_string();
        let home = env_var("HOME").unwrap_or_else(|_| ["/home/", user.as_str()].concat());
        let na = String::from("no args");
        let share_dir = [&home, "/.local/share/crust"].concat();
        let cd_prev_dir = None;
        let shell_state = ShellState {
            args,
            prompt,
            user,
            home,
            na,
            share_dir,
            cd_prev_dir,
        };
        ensure_directory(&shell_state.share_dir, true).unwrap();
        shell_state
    }
    pub fn eval_prompt(&mut self) -> String {
        let mut evaled_prompt = self.prompt.clone();
        let commands = get_commands_from_input(&self.prompt);
        let mut command_output: String;
        for command in commands {
            if command.args.contains(&String::from("|")) {
                command_output = piped_cmd(&PipedShellCommand::from(&command));
            } else {
                command_output = cmd(&command);
            }
            evaled_prompt = evaled_prompt.replace(
                format!("%({})", command.to_string().trim()).as_str(),
                command_output.trim(),
            );
        }
        let colors = crate::prompt::get_colors_from_input(&self.prompt);
        for color in colors {
            match color {
                Color::Fg(fg) => {
                    evaled_prompt = evaled_prompt.replace(
                        format!("F<{}>", fg.to_str()).as_str(),
                        format!("{}", fg.to_string()).as_str(),
                    );
                },
                Color::Bg(bg) => {
                    evaled_prompt = evaled_prompt.replace(
                        format!("B<{}>", bg.to_str()).as_str(),
                        format!("{}", bg.to_string()).as_str(),
                    );
                }
            }
        }
       // for fgcolor in &fgcolors {
       //     println!("fgcolor = {:#?}", fgcolor);
       //     evaled_prompt = evaled_prompt.replace(
       //         format!("F<{}>", fgcolor.to_str()).as_str(),
       //         format!("{}", fgcolor.to_string()).as_str(),
       //     );
       // }
       // for bgcolor in &bgcolors {
       //     println!("bgcolor = {:#?}", bgcolor);
       //     evaled_prompt = evaled_prompt.replace(
       //         format!("B<{}>", bgcolor.to_str()).as_str(),
       //         format!("{}", bgcolor.to_string()).as_str(),
       //     );
       // }
        // TODO: Add support for more escape sequences.
        // To match an escape sequence, we need to match for an escaped version of the sequence,
        // and then replace the escaped version with the actual sequence.
        // This is because the escape sequence is a single character, and the actual sequence
        // is escaped by the compiler in a user-supplied string literal.
        let substitutions = vec![
            "%{C}", "%{D12}", "%{D24}", "%{H}", "%{U}", // Information-related variables
            "\\n", // Explicit escape sequences
            "%{i}", "%{u}", // Font effect variables
            "%{re}", "%{rf}", "%{rb}" // Reset-related variables
        ];
        for to_subst in substitutions {
            let mut subst = String::new();
            match to_subst {
                "%{C}" => subst = std::env::current_dir().unwrap().display().to_string(),
                "%{D12}" => subst = get_time("%I:%M %p").to_string(),
                "%{D24}" => subst = get_time("%H:%M").to_string(),
                "%{H}" => subst = self.home.clone(),
                "%{U}" => subst = self.user.clone(),
                "\\n" => subst = '\n'.to_string(), // Needed to support newlines in the prompt
                "%{i}" => subst = "\x1b[3m".to_string(), // Italicize text
                "%{re}" => subst = "\x1b[0m".to_string(), // Reset all attributes
                "%{rf}" => subst = "\x1b[39m".to_string(), // Reset to default text color
                "%{rb}" => subst = "\x1b[49m".to_string(), // Reset to default background color
                "%{u}" => subst = "\x1b[4m".to_string(), // Underline test
                _ => (),
            }
            evaled_prompt = evaled_prompt.replace(to_subst, &subst);
        }
        evaled_prompt
    }
}

/// Tokenizes the input, returning a vector of every character in `input`.
pub fn tokenize(input: &str) -> Vec<String> {
    input.chars().map(|t| t.to_string()).collect::<Vec<String>>()
}

fn push_to_vec(from_vec: &mut Vec<String>, to_vec: &mut Vec<String>) {
    let element = from_vec.concat();
    // Don't push to the vector if element is empty.
    if element.is_empty() {
        return;
    }
    to_vec.push(element);
    from_vec.clear();
}

/// Creates a lexified vector from a tokenized one.
/// Example, if the tokenized vec was:
/// ```
/// ["e", "c", "h", "o",
///  " ",
///  "\"", "a", "r", "g", " ", "1" "\"",
///  " ",
///  "\"", "a", "r", "g", " ", "2" "\""]
/// ```
/// It would return:
/// `["echo", "arg 1", "arg 2"]`
pub fn lex_tokenized_input(input: &str) -> Vec<String> {
    let tokenized_vec = tokenize(input);
    // This is the final vector that will be returned.
    let mut lexed_vec: Vec<String> = Vec::new();
    // This is a temporary vec that gets pushed to lexed_vec.
    let mut tmp_vec: Vec<String> = Vec::new();
    // Same as tmp_vec except this is for anything in quotes.
    let mut quoted_vec: Vec<String> = Vec::new();
    // These two bools are used for checking if the character is in quotes,
    // and if the quotes part of the match statement was ran.
    let mut quoted = false;
    let mut quotes_ran = false;
    for (idx, character) in tokenized_vec.iter().enumerate() {
        match character.as_str() {
            "\"" | "'" => {
                if quotes_ran {
                    push_to_vec(&mut quoted_vec, &mut lexed_vec);
                    quoted = false;
                    quotes_ran = false;
                } else {
                    quoted = true;
                    quotes_ran = true;
                }
            }
            " " => {
                if quoted {
                    quoted_vec.push(character.to_string());
                } else {
                    push_to_vec(&mut tmp_vec, &mut lexed_vec);
                }
            }
            // Instead of explicitely checking for everything,
            // don't we just append any character that doesn't
            // require extra work, such as quotations.
            _ => {
                if quoted {
                    quoted_vec.push(character.to_string());
                } else {
                    tmp_vec.push(character.to_string());
                    // Needed to push the last element to lexed_vec.
                    if idx == tokenized_vec.len() - 1 {
                        push_to_vec(&mut tmp_vec, &mut lexed_vec);
                    }
                }
            }
        }
    }
    lexed_vec
}
