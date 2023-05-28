pub fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut echo = args.join(" ");
    for (key, value) in std::env::vars() {
        echo = echo.replace(&key, &value);
    }
    println!("{echo}")
}