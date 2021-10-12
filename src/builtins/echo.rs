use crate::shared_functions::{
    piped_cmd, PipedShellCommand, Redirection, ShellCommand
};

/// Just like you know it. Takes the args part of ShellCommand and prints them.
pub fn echo(args: Vec<String>) -> String {
    let mut output = String::new();
    if args.contains(&"|".to_string()) {
        let command = ShellCommand {
            name: "echo".to_string(),
            args,
            redirection: Redirection::NoOp,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">>".to_string()) {
        let command = ShellCommand {
            name: "echo".to_string(),
            args,
            redirection: Redirection::Append,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">".to_string()) {
        let command = ShellCommand {
            name: "echo".to_string(),
            args,
            redirection: Redirection::Overwrite,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else {
        for arg in args {
            output.push_str(format!("{} ", arg).as_str());
        }
    }
    output
}
