use anyhow::Context;

mod power_set;
mod publish;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    #[clap(alias = "powerset")]
    PowerSet(power_set::PowerSet),
    Publish(publish::Publish),
}

impl Commands {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Commands::PowerSet(cmd) => cmd.run().context("power set"),
            Commands::Publish(cmd) => cmd.run().context("publish"),
        }
    }
}
