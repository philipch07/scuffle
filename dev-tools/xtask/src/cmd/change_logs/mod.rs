use anyhow::Context;

mod check_pr;
mod generate;
mod util;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    Generate(generate::Generate),
    CheckPr(check_pr::CheckPr),
}

impl Commands {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Commands::Generate(cmd) => cmd.run().context("generate change logs"),
            Commands::CheckPr(cmd) => cmd.run().context("check pr"),
        }
    }
}
