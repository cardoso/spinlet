use spinlets::*;

const PROMPT: &str = "> ";

fn main() {
    if cfg!(not(target_arch = "wasm32")) {
        println!("Running this natively is not a good idea!");
        return;
    }

    let mut spin = Spinlet::get();

    loop {
        let input = prompt(&spin);
        
        if should_exit(&input) { break; }

        execute(&mut spin, input);
    }
}

fn execute(spin: &mut Spinlet, input: String) {
    let output = eval(spin, &input);
    print_output(spin, output);
}

fn prompt(spin: &Spinlet) -> String {
    print_prompt(spin);
    read_line(spin)
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
        cmd => unknown(cmd)
    }
}

fn unknown(command: &str) -> String {
    format!("Unknown command: {}", command)
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
        None => format!("No file specified")
    }
}

fn pwd(spin: &mut Spinlet) -> String {
    match spin.workspace_mut().pwd() {
        Ok(dir) => dir,
        Err(e) => format!("Failed to get current directory: {}", e)
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
        Some(dir) => match spin.workspace_mut().cd(dir) {
            Ok(dir) => format!("Changed directory to {}", dir),
            Err(e) => format!("Failed to change directory: {}", e)
        },
        None => match spin.workspace_mut().cd("/") {
            Ok(dir) => format!("Changed directory to {}", dir),
            Err(e) => format!("Failed to change directory: {}", e)
        }
    }
}