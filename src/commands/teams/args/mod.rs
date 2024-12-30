pub(crate) mod list;

use crate::commands::teams::args::list::TeamListCommand;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TeamCommand {
    #[command(subcommand)]
    pub command: TeamSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum TeamSubCommand {
    /// List teams
    List(TeamListCommand),
}
