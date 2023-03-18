use clap::Args;

#[derive(Debug, Args)]
pub struct RunCommand {
    /// name of app
    pub name: String,
}

pub fn run_command() {}
