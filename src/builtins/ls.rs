use crate::commands::is_piped;

/// List dir entries. Take the args part of `ShellCommand`.
pub fn ls(mut args: Vec<String>) -> String {
    let mut output = String::new();
    if args.is_empty() {
        args.push(".".to_string());
    }
    is_piped(&args, "ls");
    let mut path_idx = 0;
    for (idx, arg) in args.iter().enumerate() {
        if !arg.starts_with("--") || !arg.starts_with('-') {
            path_idx = idx;
        }
    }
    let input = &args[path_idx];
    let path;
    if std::path::Path::new(input).exists() {
        path = std::fs::read_dir(input).unwrap();
    } else {
        println!("ERROR: '{}' is not a valid file or directory.", input);
        return String::from("");
    }

    for file in path {
        let raw_entry = file.unwrap().path();
        #[cfg(any(target_os = "android", target_os = "linux"))]
        let still_raw_entry = raw_entry.to_str().unwrap().replace("./", "");
        #[cfg(target_os = "windows")]
        let still_raw_entry = raw_entry.to_str().unwrap().replace(".\\", "");
        let paths = still_raw_entry.split('\n');
        for line in paths {
            #[cfg(any(target_os = "android", target_os = "linux"))]
            let parts = line.split('/');
            #[cfg(target_os = "windows")]
            let parts = line.split('\\');
            let mut n = 0;
            #[cfg(any(target_os = "android", target_os = "linux"))]
            let parts_count = line.split('/').count();
            #[cfg(target_os = "windows")]
            let parts_count = line.split('\\').count();
            for part in parts {
                #[cfg(feature = "colors")]
                {
                    #[cfg(feature = "colors")]
                    use colored::Colorize;
                    if part.starts_with('.') || n == parts_count - 1 {
                        #[cfg(feature = "colors")]
                        output.push_str(format!("{}", part.white()).as_str());    
                    } else {
                        #[cfg(feature = "colors")]
                        output.push_str(format!("{}", part.green()).as_str());
                    }
                }
                #[cfg(not(feature = "colors"))]
                output.push_str(&part.to_string());

                n += 1;
                if n == parts_count {
                    break;
                }
                output.push('/');
            }
            output.push('\n');
        }
    }
    output
}
