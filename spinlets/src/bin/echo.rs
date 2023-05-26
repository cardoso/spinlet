fn main() {
    let env = std::env::args().collect::<Vec<_>>();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut echo = args.join(" ");
    for env in env {
        echo = echo.replace(&env, &std::env::var(&env).unwrap_or("".into()));
    }
    

    println!("{echo}")
}