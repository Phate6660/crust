use crate::commands::is_piped;
use sflib::{line, read};

pub fn cat(args: &[String]) -> String {
    is_piped(args, "cat");
    match args[0].as_str() {
        "-l" => {
            let line_number = args[1].parse::<usize>().unwrap() - 1; // -1 to account for 0-indexing.
            line(&args[2], line_number).unwrap()
        }
        "-n" => {
            let mut final_output = String::new();
            let output = read(&args[1]).unwrap();
            let output_vec = output.split('\n');
            for (idx, line) in output_vec.enumerate() {
                let string = format!("{} {}\n", idx, line);
                final_output.push_str(&string);
            }
            final_output
        }
        _ => read(&args[0]).unwrap(),
    }
}
