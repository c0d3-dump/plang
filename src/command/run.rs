use clap::Args;

#[derive(Debug, Args)]
pub struct RunCommand {
    /// enter name of app
    pub name: String,
}
