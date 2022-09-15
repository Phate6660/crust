mod builtins;
mod commands;
mod prompt;
mod shared_functions;
mod tests;

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
    // bell style="nothing"
    // edit mode="emacs"
    // history auto add lines=true
    // history file="`shell_state.history`"
    // history size=500
    // history spaces ignored=true
    // prompt="`shell_state.prompt`"
    // should be invalid="N/A"
    // ```
    let default_config = format!(
        "bell style=\"{}\"\nedit mode=\"{}\"\nhistory auto add lines={}\nhistory file=\"{}\"\nhistory size={}\nprompt=\"{}\"\nshould be invalid=\"N/A\"",
        &shell_state.bell_style,
        &shell_state.edit_mode,
        &shell_state.history_auto_add_lines,
        &shell_state.history_file,
        &shell_state.history_size,
        &shell_state.prompt
    );
    #[cfg(feature = "readline")]
    let options = conf::get_options(shell_state.config.as_str(), &default_config);
    #[cfg(feature = "readline")]
    if let Ok(options) = options {
        for option in options {
            match option.0.as_str() {
                "bell style" => shell_state.bell_style = option.1,
                "edit mode" => shell_state.edit_mode = option.1,
                "history auto add lines" => shell_state.history_auto_add_lines = option.1.parse::<bool>().unwrap(),
                "history file" => shell_state.history_file = option.1,
                "history size" => shell_state.history_size = option.1.parse::<usize>().unwrap(),
                "history spaces ignored" => shell_state.history_spaces_ignored = option.1.parse::<bool>().unwrap(),
                "prompt" => shell_state.prompt = option.1,
                _ => println!("[WARNING]: '{}' is an invalid option, ignoring.", option.0)
            }
        }
    }
    #[cfg(feature = "readline")]
    let bell_style: rustyline::config::BellStyle = match shell_state.bell_style.as_str() {
        "nothing" => rustyline::config::BellStyle::None,
        "bell" => rustyline::config::BellStyle::Audible,
        "flashing" => rustyline::config::BellStyle::Visible,
        _ => rustyline::config::BellStyle::None
    };
    #[cfg(feature = "readline")]
    let edit_mode: rustyline::EditMode = match shell_state.edit_mode.as_str() {
        "emacs" => rustyline::EditMode::Emacs,
        "vi" => rustyline::EditMode::Vi,
        _ => rustyline::EditMode::Emacs
    };
    #[cfg(feature = "readline")]
    let config = rustyline::Config::builder()
        .auto_add_history(shell_state.history_auto_add_lines)
        .bell_style(bell_style)
        .edit_mode(edit_mode)
        .history_ignore_space(shell_state.history_spaces_ignored)
        .max_history_size(shell_state.history_size)
        .build();
    non_interactive(&mut shell_state);
    #[cfg(feature = "readline")]
    let mut rl = Editor::with_config(config);
    #[cfg(feature = "readline")]
    if rl.load_history(&shell_state.history_file).is_err() {
        println!("There was no previous history to load.");
    }
    #[cfg(not(feature = "readline"))]
    run_loop(shell_state);
    #[cfg(feature = "readline")]
    run_loop(&mut rl, shell_state);
}
