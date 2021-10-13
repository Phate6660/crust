use crate::shared_functions::{ShellCommand, ShellState};
use std::env::current_dir;

/// Helper for cd, to actually change the dirctory.
fn cd_helper(dir: &str) {
    let path = std::path::Path::new(dir);
    match std::env::set_current_dir(&path) {
        | Ok(()) => (),
        | Err(_) => println!("Failed to change directory to '{}'", path.display())
    }
}

/// Used to change directory.
/// Takes a ShellState and ShellCommand.
/// ShellState is used to realize `cd -` fuctionality,
/// but can be used for other options in the future.
pub fn cd(shell_state: &mut ShellState, command: ShellCommand) {
    if command.args.is_empty() {
        shell_state.cd_prev_dir = Some(current_dir().unwrap());
        let user = std::env::var("USER").unwrap();
        let home = ["/home/", user.as_str()].concat();
        cd_helper(&home);
    } else if command.args[0] == "-" {
        if shell_state.cd_prev_dir.is_none() {
            println!("No previous dir found");
            return;
        }
        // unwrap can be safely used here, because function would've returned
        // if cd_prev_dir is None
        match &shell_state.cd_prev_dir.as_ref().unwrap().to_str() {
            | Some(path) => cd_helper(path),
            | None => {
                println!("Could not convert Path to String (src/buildins.rs in function cd)");
                shell_state.cd_prev_dir = None;
            }
        }
        shell_state.cd_prev_dir = Some(current_dir().unwrap());
    } else {
        shell_state.cd_prev_dir = Some(current_dir().unwrap());
        cd_helper(&command.args[0]);
    }
}
