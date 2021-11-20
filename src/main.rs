mod builtins;
mod commands;
mod shared_functions;

#[cfg(feature = "readline")]
use rustyline::Editor;
use shared_functions::{non_interactive, run_loop, ShellState};

fn main() {
    let mut shell_state = ShellState::init();
    let prompt = ShellState::eval_prompt(&mut shell_state);
    non_interactive(&mut shell_state);
    #[cfg(feature = "readline")]
    let mut rl = Editor::<()>::new();
    #[cfg(feature = "readline")]
    let history_file = [shell_state.share_dir.as_str(), "/crust.history"].concat();
    #[cfg(feature = "readline")]
    if rl.load_history(&history_file).is_err() {
        println!("There was no previous history to load.");
    }
    #[cfg(not(feature = "readline"))]
    run_loop(&prompt, shell_state);
    #[cfg(feature = "readline")]
    run_loop(&prompt, &mut rl, &history_file, shell_state);
}
