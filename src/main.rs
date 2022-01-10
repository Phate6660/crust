mod builtins;
mod commands;
mod prompt;
mod shared_functions;

#[cfg(feature = "readline")]
use rustyline::Editor;
use shared_functions::{process_input, run_loop, ShellState};

/// A function to parse input, used for the barebones prompt.
pub fn parse_input(op: &str) -> String {
    if op == "interactive" {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read user input");
        input.trim().to_string()
    } else {
        std::env::args()
            .collect::<Vec<String>>()
            .get(2)
            .unwrap()
            .replace('"', "")
            .trim()
            .to_string()
    }
}

/// A helper function to run a non-interactive command,
/// it will automatically check if `-c` was passed as an arg
/// and run commands non-interactively.
pub fn non_interactive(shell_state: &mut ShellState) {
    if shell_state.args.get(1).unwrap_or(&shell_state.na) == "-c" {
        let input = parse_input("non-interactive");
        process_input(shell_state, &input);
        std::process::exit(0);
    }
}

fn main() {
    let mut shell_state = ShellState::init();
    // Default config:
    // ```
    // history file="`shell_state.history`"
    // prompt="`shell_state.prompt`"
    // should_be_invalid="N/A"
    // ```
    let default_config = format!(
        "history file=\"{}\"\nprompt=\"{}\"\nshould_be_invalid=\"N/A\"", 
        &shell_state.history_file,
        &shell_state.prompt
    );
    let options = conf::get_options(shell_state.config.as_str(), &default_config);
    if let Ok(options) = options {
        for option in options {
            match option.0.as_str() {
                "history file" => shell_state.history_file = option.1,
                "prompt" => shell_state.prompt = option.1,
                _ => println!("[WARNING]: '{}' is an invalid option, ignoring.", option.0)
            }
        }
    }
    non_interactive(&mut shell_state);
    #[cfg(feature = "readline")]
    let mut rl = Editor::<()>::new();
    #[cfg(feature = "readline")]
    if rl.load_history(&shell_state.history_file).is_err() {
        println!("There was no previous history to load.");
    }
    #[cfg(not(feature = "readline"))]
    run_loop(shell_state);
    #[cfg(feature = "readline")]
    run_loop(&mut rl, shell_state);
}
