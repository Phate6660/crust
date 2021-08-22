use std::io::Write;
use std::process::Command;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let na = String::from("no args");
    if args.get(1).unwrap_or(&na) == "-c" {
        println!("Command mode!");
        std::process::exit(0);
    }
    loop {
        let crusty_prompt = String::from("[crusty]: ");
        let prompt = std::env::var("PROMPT").unwrap_or(crusty_prompt);
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = std::string::String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");
        input = input.trim().to_string();
        if input.contains(' ') {
            let input = input.split(' ').collect::<Vec<&str>>();
            let mut child = Command::new(input[0])
                .args(&input[1..])
                .spawn()
                .unwrap();
            child.wait().unwrap();
        } else {
            let mut child = Command::new(input)
                .spawn()
                .unwrap();
            child.wait().unwrap();
        };
    }
}
