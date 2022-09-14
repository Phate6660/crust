pub fn print(args: Vec<String>) -> String {
    let dir = if args.get(1) == None || args[1] == "." {
        let dir = std::env::current_dir().unwrap();
        let dir = dir.to_string_lossy();
        dir.to_string()
    } else {
        args.get(1).unwrap().to_string()
    };
    let dir_vec = dir.split('/').collect::<Vec<&str>>();
    let dir_vec_count = dir_vec.iter().count();
    let mut n=0;
    let mut output = String::new();
    for segment in dir_vec {
        if n == 0 {
            n = n + 1;
            continue;
        }
        if n < dir_vec_count - 1 {
            let char = segment.chars().collect::<Vec<char>>()[0];
            output = output + &format!("/{}", char).to_string();
        } else if n == dir_vec_count - 1 {
            output = output + &format!("/{}", segment).to_string();
        }
        n = n + 1;
    }
    return output.to_string();
}
