use crate::shared_functions::is_piped;

/// Just like you know it. Takes the args part of `ShellCommand` and prints them.
pub fn echo(args: &[String]) -> String {
    let mut output = String::new();
    is_piped(&args, "echo");
    for arg in args {
        // TODO: Support other escape sequences.
        let arg_to_push = arg.replace("\\n", "\n"); // Needed to replace \n with newline.
        output.push_str(format!("{} ", arg_to_push).as_str());
    }
    output
}
