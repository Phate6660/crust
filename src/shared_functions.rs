use crate::commands::{cmd_with_output, piped_cmd, return_shellcommand, PipedShellCommand, Redirection, ShellCommand};
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
    pub config_dir: String,
    pub config: String,
    pub history_file: String,
    pub edit_mode: String,
    pub bell_style: String,
    pub history_auto_add_lines: bool,
    pub history_size: usize,
    pub history_spaces_ignored: bool,
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
pub fn run_loop(rl: &mut Editor<()>, mut shell_state: ShellState) {
    loop {
        let prompt = rl.readline(&ShellState::eval_prompt(&mut shell_state));
        match prompt {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.starts_with("exit") {
                    if line.contains(' ') {
                        let input = line.split(' ').collect::<Vec<&str>>()[1];
                        rl.save_history(&shell_state.history_file).unwrap();
                        exit(input.parse::<i32>().unwrap_or(0));
                    } else {
                        rl.save_history(&shell_state.history_file).unwrap();
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
    rl.save_history(&shell_state.history_file).unwrap();
}

#[cfg(not(feature = "readline"))]
pub fn run_loop(mut shell_state: ShellState) {
    loop {
        let prompt = ShellState::eval_prompt(&mut shell_state);
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let input = crate::parse_input("interactive");
        process_input(&mut shell_state, &input);
    }
}

impl ShellState {
    /// Initalizes the shell state with all the informations needed.
    ///
    /// `cd_prev_dir` doesnt hold a value, because there is no previous dir yet.
    pub fn init() -> ShellState {
        let args = std::env::args().collect();
        let prompt = env_var("PROMPT").unwrap_or_else(|_| String::from("F<GREEN>B<BLACK>%{b}%{u}[crust]-[%{CL}]:%{re} "));
        let user_command = return_shellcommand(String::from("whoami"), Vec::new(), Redirection::NoOp);
        let user = env_var("USER").unwrap_or_else(|_| cmd_with_output(&user_command)).trim().to_string();
        let home = env_var("HOME").unwrap_or_else(|_| ["/home/", user.as_str()].concat());
        let na = String::from("no args");
        let share_dir = [&home, "/.local/share/crust"].concat();
        let cd_prev_dir = None;
        let config_dir = [&home, "/.config/crust/"].concat();
        let config = [&config_dir, "config"].concat();
        let history_file = [&share_dir, "/crust.history"].concat();
        let shell_state = ShellState {
            args,
            prompt,
            user,
            home,
            na,
            share_dir,
            cd_prev_dir,
            config_dir,
            config,
            history_file,
            edit_mode: String::from("emacs"),
            bell_style: String::from("nothing"),
            history_auto_add_lines: true,
            history_size: 500,
            history_spaces_ignored: true,
        };
        ensure_directory(&shell_state.share_dir, true).unwrap();
        ensure_directory(&shell_state.config_dir, true).unwrap();
        shell_state
    }
    pub fn eval_prompt(&mut self) -> String {
        let mut evaled_prompt = self.prompt.clone();
        let commands = crate::prompt::get_commands_from_input(&self.prompt);
        let mut command_output: String;
        for command in commands {
            if command.args.contains(&String::from("|")) {
                command_output = piped_cmd(&PipedShellCommand::from(&command));
            } else {
                command_output = cmd_with_output(&command);
            }
            evaled_prompt = evaled_prompt.replace(
                format!("%({})", command.to_string().trim()).as_str(),
                command_output.trim(),
            );
        }
        let files = crate::prompt::get_files_from_input(&evaled_prompt);
        for file in files {
            evaled_prompt = evaled_prompt.replace(
                format!("%[{}]", file.to_string().trim()).as_str(),
                crate::builtins::cat::cat(&[file]).trim(),
            );
        }
        // Parse the prompt and replace the colors with the escape sequences.
        evaled_prompt = crate::prompt::parse_prompt_effects(&evaled_prompt);
        let substitutions = vec!["%{CL}", "%{CS}", "%{D}", "%{H}", "%{T12}", "%{T24}", "%{U}", "\\n"];
        for to_subst in substitutions {
            let mut subst = String::new();
            match to_subst {
                "%{CL}" => subst = std::env::current_dir().unwrap().display().to_string(),
                "%{CS}" => {
                    let cwd = std::env::current_dir().unwrap().display().to_string();
                    subst = cwd.split('/').collect::<Vec<&str>>()[cwd.split('/').count() - 1].to_string();
                },
                "%{D}" => subst = get_time("%a %b %d"),
                "%{H}" => subst = self.home.clone(),
                "%{T12}" => subst = get_time("%I:%M %p").to_string(),
                "%{T24}" => subst = get_time("%H:%M").to_string(),
                "%{U}" => subst = self.user.clone(),
                "\\n" => subst = '\n'.to_string(), // Needed to support newlines in the prompt
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
