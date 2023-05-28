use spinlets::*;

pub fn main() -> Result<()> {
    let spin = Spinlet::get();
    spin.console().print("new")?;
    Ok(())
}