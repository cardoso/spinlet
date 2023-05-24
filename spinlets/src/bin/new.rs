use spinlets::*;

pub fn main() -> Result<()> {
    let spin = Spin::get();

    spin.console().print("new")?;


    Ok(())
}