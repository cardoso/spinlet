mod spinlet;
mod cli;

pub use clap::Parser;
use spinlet::*;
pub use cli::*;
use anyhow::Result;

impl Cli {
    pub async fn run(&self) -> Result<()> {

        let command = SpinletCtx::new(self.args(), &self.envs())?;
        let mut spinlet = Spinlet::load(self.path(), command).await?;
        spinlet.run().await
    }
}

