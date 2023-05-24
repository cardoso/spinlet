use anyhow::Context;

use spinlets::*;

fn main() -> Result<()> {
    let mut spin = Spin::get()?;



    loop {
        write!(spin.stderr, "> ").context("Could not write prompt")?;
        spin.stderr.flush().context("Could not flush")?;

        let buffer = &mut String::new();
        spin.stdin.read_line(buffer).context("Could not read line")?;

        write!(spin.stdout, "{buffer}").context("Could not write line")?;
    }
    Ok(())
}
