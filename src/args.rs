use clap::{Parser, Subcommand};

use crate::command::RunCommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PcliArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// run app
    Run(RunCommand),

    /// list all installed apps
    List,

    /// search apps from registry
    Search,
}
