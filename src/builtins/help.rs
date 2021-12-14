/// Prints a list of builtin commands.
pub fn help(args: &[String]) {
    if args.is_empty() {
        println!("\
            crust [https://github.com/Phate6660/crust]\n\
            builtins:\n\
            ---------\n\
            calc\n\
            cd\n\
            echo\n\
            exit\n\
            help\n\
            ls\n\
            pwd\n\
            prompt"
        );
        return;
    }
    if args.len() > 1 {
        println!("Please specify only one command.");
        return;
    }
    match args[0].as_str() {
        "calc" => println!("Supports +, -, /, x for two numbers.\nE.g. 1+1, 4/2, 2x4, 2-1"),
        "cat" => println!("Support printing file output, and additionally showing line numbers with `-n`."),
        "cd" => {
            println!("\
                Takes a absolute or relative path and changes directory to it.\n\
                `cd -` will take you to your previous dir."
            );
        }
        "echo" => println!("Takes n amount of arguments and prints them to stdout."),
        "exit" => println!("Exits the shell with the given exit code."),
        "help" => println!("Returns information about the builtin commands."),
        "ls" => println!("Lists the content of a directory."),
        "pwd" => println!("Prints the working directory."),
        "prompt" => {
            println!("\
                Can be set to a static string, by just setting the string in the PROMPT env variable,\n\
                or can be set to a dynamic prompt, by including a command to be executed, by delimiting it with %(),\
                in the prompt string.\n\
                Variables are also supported, such as %{{U}} for user, %{{C}} for CWD, etc.\n\
                e.G.: `%(whoami)E@%(hostname)> `. This will, for my case, produce `zeno@aether> `."
            );
        }
        _ => {
            println!("\
                cRUSTy [https://github.com/Phate6660/crust]\n\
                builtins:\n\
                ---------\n\
                calc\n\
                cd\n\
                echo\n\
                exit\n\
                help\n\
                ls\n\
                pwd\n\
                prompt"
            );
        }
    }
}
