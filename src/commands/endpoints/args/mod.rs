pub(crate) mod list;

use crate::commands::endpoints::args::list::EndpointListCommand;
use clap::{Args, Subcommand};

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
