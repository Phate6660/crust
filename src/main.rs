mod builtins;
mod shared_functions;

use builtins::{calc, cd, echo, help, ls};
#[cfg(feature = "readline")]
use rustyline::{error::ReadlineError, Editor};
use std::process::exit;
use shared_functions::{cmd, ShellState, non_interactive, piped_cmd, piped_text};

#[cfg(not(feature = "readline"))]
use shared_functions::parse_input;

// Process the input to run the appropriate builtin or external command.
fn process_input(shell_state: &mut ShellState, input: String) {
    if input.starts_with("calc") {
        calc(&input);
    } else if input.starts_with("cd") {
        cd(shell_state, &input);
    } else if input.starts_with("echo") {
        echo(&input);
    } else if input.starts_with("help") {
        help();
    } else if input.starts_with("ls") {
        ls(&input);
    } else if input == "pwd" {
        println!("{}", std::env::current_dir().unwrap().display());
    } else if input.contains('|') {
        piped_cmd(&input);
    } else if input.contains(' ') {
        cmd(&input, true);
    } else {
        cmd(&input, false);
    }
}

#[cfg(feature = "readline")]
fn main() {
    let mut shell_state = ShellState::init();
    non_interactive(&mut shell_state);
    let mut rl = Editor::<()>::new();
    let history_file = [shell_state.share_dir.as_str(), "/crusty.history"].concat();
    if rl.load_history(&history_file).is_err() {
        println!("There was no previous history to load.");
    }
    loop {
        let prompt = rl.readline(&shell_state.prompt);
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
                process_input(&mut shell_state, line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
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
fn main() {
    let shell_state = ShellState::new();
    non_interactive(&mut shell_state);
    loop {
        print!("{}", shell_state.prompt);
        std::io::stdout().flush().unwrap();
        let input = parse_input("interactive");
        process_input(&mut shell_state, input);
    }
}
