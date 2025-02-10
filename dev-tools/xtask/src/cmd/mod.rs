use anyhow::Context;

mod power_set;
mod workspace_deps;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    #[clap(alias = "powerset")]
    PowerSet(power_set::PowerSet),
    WorkspaceDeps(workspace_deps::WorkspaceDeps),
}

impl Commands {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Commands::PowerSet(cmd) => cmd.run().context("power set"),
            Commands::WorkspaceDeps(cmd) => cmd.run().context("workspace deps"),
        }
    }
}
