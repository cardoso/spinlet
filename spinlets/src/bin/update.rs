
fn main() {
    match std::fs::read_dir("/workspace") {
        Ok(dir) => for entry in dir {
            match entry {
                Ok(entry) => println!("{}", entry.path().display()),
                Err(error) => println!("error reading entry: {}", error),
            }
        }
        Err(error) => println!("error reading /: {}", error),
    }

    for arg in std::env::args() {
        println!("{}", arg);
    }

    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
