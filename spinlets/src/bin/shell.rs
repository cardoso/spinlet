use anyhow::{anyhow, bail};
use spinlets::{*, vfs::Vfs};

fn main() -> Result<()> {
    let mut spin = Spin::get()?;

    loop {
        spin.console().print("> ")?;

        let input = spin.console().read_line()?;
        
        if input.trim() == "exit" {
            break;
        }
        
        let output = match parse(&mut spin.vfs_mut(), &input) {
            Ok(output) => output,
            Err(e) => format!("{e}"),
        };
        
        spin.console().print_line(output)?;
    }

    Ok(())
}

fn parse(vfs: &mut Vfs, input: &str) -> Result<String> {
    let mut args = input.split_whitespace();
    let command = args.next().expect("No command");
    match command {
        "cd" => {
            match args.next() {
                Some(dir) => vfs.cd(dir),
                None => vfs.cd("/"),
            }?;

            Ok("".into())
        }
        "ls" => vfs.ls().map(|s| s.into_iter().map(|e| e.display().to_string()).collect::<Vec<String>>().join("\n")),
        "pwd" => vfs.pwd(),
        _ => bail!("Unknown command: {}", command)
    }
}