use spinlets::*;

const PROMPT: &str = "$ ";
const HELP: &&str = &"
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
    spin.console().print_line(&output).expect("Failed to print output");
}

fn read_line(spin: &Spinlet) -> String {
    spin.console().read_line().expect("Failed to read line")
}

fn print_prompt(spin: &Spinlet) {
    spin.console().print(PROMPT).expect("Failed to print prompt");
}

fn eval(spin: &mut Spinlet, input: &str) -> String {
    let mut args = input.split_whitespace();
    let Some(command) = args.next() else { return "".to_string() };
    match command {
        "cd" => cd(&mut args, spin),
        "ls" => ls(spin),
        "pwd" => pwd(spin),
        "cat" => cat(args, spin),
        "env" => env(),
        "help" => help(spin, input),
        cmd => unknown(cmd)
    }
}

fn help(spin: &Spinlet, input: &str) -> String {
    let root = spin.workspace().root().display();
    
    
    let output = match input {
        "help" => HELP.to_string(),
        "help cd" => "cd <dir> - change directory".to_string(),
        "help ls" => "ls - list files".to_string(),
        "help pwd" => "pwd - print working directory".to_string(),
        "help cat" => "cat <file> - print file contents".to_string(),
        "help env" => "env - print environment variables".to_string(),
        "help help" => "help - print this help message".to_string(),
        "help exit" => "exit - exit the shell".to_string(),
        input => format!("Unknown help topic: {input}")
    };

    format!("[{root}] {output}")
}

fn unknown(command: &str) -> String {
    format!("Unknown command: {}", command)
}

fn env() -> String {
    std::env::vars().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("\n")
}

fn cat(mut args: std::str::SplitWhitespace, spin: &mut Spinlet) -> String {
    match args.next() {
        Some(file) => match spin.workspace_mut().cat(file) {
            Ok(content) => content,
            Err(e) => format!("Failed to read file: {e}")
        },
        None => "No file specified".to_string()
    }
}

fn pwd(spin: &mut Spinlet) -> String {
    match spin.workspace_mut().pwd() {
        Ok(dir) => dir,
        Err(e) => format!("Failed to get current directory: {e}")
    }
}

fn ls(spin: &mut Spinlet) -> String {
    match spin.workspace_mut().ls() {
        Ok(files) => files.iter().flat_map(|file| file.file_name()?.to_str()).collect::<Vec<_>>().join("\n"),
        Err(e) => format!("Failed to list files: {}", e)
    }
}

fn cd(args: &mut std::str::SplitWhitespace, spin: &mut Spinlet) -> String {
    match args.next() {
        Some(dir) => spin.workspace_mut().cd(dir),
        None => spin.workspace_mut().cd("/")
    }
}