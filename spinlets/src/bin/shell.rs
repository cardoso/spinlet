use spinlets::*;

const PROMPT: &str = " $ ";
const HELP: &str = "
    cd <dir> - change directory,
    ls - list files,
    pwd - print working directory,
    cat <file> - print file contents,
    env - print environment variables,
    help - print this help message,
    exit - exit the shell
";

fn main() {
    if cfg!(not(target_arch = "wasm32")) {
        println!("Running this natively is not a good idea!");
        return;
    }

    let mut spin = Spinlet::get();

    loop {
        print_prompt(&spin);
        let input = read_line(&spin);
        if should_exit(&input) { break; }
        let output = eval(&mut spin, &input);
        print_output(&spin, output);
    }
}

fn should_exit(input: &str) -> bool {
    matches!(input.trim(), "exit" | "quit")
}

fn print_output(spin: &Spinlet, output: String) {
    spin.console().print_line(output).expect("Failed to print output");
}

fn read_line(spin: &Spinlet) -> String {
    spin.console().read_line().expect("Failed to read line")
}

fn print_prompt(spin: &Spinlet) {
    spin.console().print(pwd(spin)).expect("Failed to print pwd");
    spin.console().print(PROMPT).expect("Failed to print prompt");
}

fn eval(spin: &mut Spinlet, input: &str) -> String {
    let mut args = input.split_whitespace();
    let Some(command) = args.next() else { return "".to_string() };
    match command {
        "cd" => cd(args, spin),
        "ls" => ls(spin),
        "pwd" => pwd(spin),
        "cat" => cat(args, spin),
        "env" => env(),
        "help" => help(args, spin),
        "toml" => toml(args, spin),
        cmd => unknown(cmd)
    }
}

fn toml(mut args: std::str::SplitWhitespace, spin: &Spinlet) -> String {
    let input = match args.next() {
        Some(input) => input,
        None => return "No toml path provided".to_string()
    };

    let toml = match spin.workspace().toml(input) {
        Ok(toml) => toml,
        Err(e) => return format!("Failed to read toml: {e}")
    };

    if let Some(path) = args.next() {
        let parts = path.split('.');

        let mut value = toml.as_item();

        for part in parts {
            if let Some(v) =  value.get(part) {
                value = v;
            } else {
                return format!("No value found for path: {path}");
            }
        }
        
        format!("{value}")
    } else {
        format!("{toml}")
    }
}

fn help(mut args: std::str::SplitWhitespace, spin: &Spinlet) -> String {
    let Some(input) = args.next() else {
        return HELP.to_string()
    };


    match input {
        "help" => HELP.to_string(),
        "cd" => "cd <dir> - change directory".to_string(),
        "ls" => "ls - list files".to_string(),
        "pwd" => "pwd - print working directory".to_string(),
        "cat" => "cat <file> - print file contents".to_string(),
        "env" => "env - print environment variables".to_string(),
        "toml" => "toml <file> <path?> - print toml file contents".to_string(),
        "exit" => "exit - exit the shell".to_string(),
        input => format!("Unknown help topic: {input}")
    }
}

fn unknown(command: &str) -> String {
    format!("Unknown command: {command}")
}

fn env() -> String {
    std::env::vars().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("\n")
}

fn cat(mut args: std::str::SplitWhitespace, spin: &mut Spinlet) -> String {
    match args.next() {
        Some(file) => match spin.workspace_mut().cat(file) {
            Ok(content) => content,
            Err(e) => format!("Failed to read file: {}", e)
        },
        None => "No file specified".to_string()
    }
}

fn pwd(spin: &Spinlet) -> String {
    match spin.workspace().pwd() {
        Ok(dir) => dir,
        Err(e) => format!("Failed to get current directory: {}", e)
    }
}

fn ls(spin: &mut Spinlet) -> String {
    match spin.workspace().ls() {
        Ok(files) => files.iter().flat_map(|file| file.file_name()?.to_str()).collect::<Vec<_>>().join("\n"),
        Err(e) => format!("Failed to list files: {}", e)
    }
}

fn cd(mut args: std::str::SplitWhitespace, spin: &mut Spinlet) -> String {
    match args.next() {
        Some(dir) => spin.workspace_mut().cd(dir),
        None => spin.workspace_mut().cd("/"),
    }
}