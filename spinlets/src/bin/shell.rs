use spinlets::*;

fn main() {
    let mut spin = Spinlet::get();

    loop {
        spin.console().print("> ").expect("Failed to print prompt");

        let input = spin.console().read_line().expect("Failed to read line");
        
        if input.trim() == "exit" {
            break;
        }

        let output = parse(spin.workspace_mut(), &input);

        spin.console().print_line(&output).expect("Failed to print output");
    }
}

fn parse(vfs: &mut Workspace, input: &str) -> String {
    let mut args = input.split_whitespace();
    let command = args.next().expect("No command");
    match command {
        "cd" => match args.next() {
            Some(dir) => match vfs.cd(dir) {
                Ok(dir) => format!("Changed directory to {}", dir),
                Err(e) => format!("Failed to change directory: {}", e)
            },
            None => match vfs.cd("/") {
                Ok(dir) => format!("Changed directory to {}", dir),
                Err(e) => format!("Failed to change directory: {}", e)
            }
        },
        "ls" => match vfs.ls() {
            Ok(files) => files.iter().flat_map(|file| file.file_name()?.to_str()).collect::<Vec<_>>().join("\n"),
            Err(e) => format!("Failed to list files: {}", e)
        },
        "pwd" => match vfs.pwd() {
            Ok(dir) => dir,
            Err(e) => format!("Failed to get current directory: {}", e)
        },
        "cat" => match args.next() {
            Some(file) => match vfs.cat(file) {
                Ok(content) => content,
                Err(e) => format!("Failed to read file: {}", e)
            },
            None => format!("No file specified")
        },
        "env" => std::env::vars().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("\n"),
        _ => format!("Unknown command: {}", command)
    }
}