use eyre::{bail, Result};

pub fn run() -> Result<()> {
    always_errors()?;
    Ok(())
}

pub fn always_errors() -> Result<()> {
    bail!("I am erroring");
}
