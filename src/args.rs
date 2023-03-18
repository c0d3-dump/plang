use clap::{Parser, Subcommand};

use crate::command::RunCommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PcliArgs {
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// run app
    Run(RunCommand),

    /// list all installed apps
    List,

    /// install app
    Install,

    /// search apps from registry
    Search,

    /// create new app
    New,
}
