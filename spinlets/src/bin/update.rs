
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
