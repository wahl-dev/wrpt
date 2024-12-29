pub(crate) mod list;

use clap::{Args, Subcommand};
use crate::commands::endpoints::args::list::EndpointListCommand;

#[derive(Debug, Args)]
pub struct EndpointCommand {
    #[command(subcommand)]
    pub command: EndpointSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum EndpointSubCommand {
    /// List endpoints
    List(EndpointListCommand),
}
