pub(crate) mod deploy;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod resource_control;

use crate::commands::stacks::args::deploy::StackDeployCommand;
use crate::commands::stacks::args::list::StackListCommand;
use crate::commands::stacks::args::remove::StackRemoveCommand;
use crate::commands::stacks::args::resource_control::StackResourceControlCommand;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct StackCommand {
    #[command(subcommand)]
    pub command: StackSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum StackSubCommand {
    /// List all stacks based on the current user authorizations
    List(StackListCommand),

    /// Deploy a stack
    Deploy(StackDeployCommand),

    /// Remove a stack
    Remove(StackRemoveCommand),

    /// Display the ResourceControl details of a specific stack
    ResourceControl(StackResourceControlCommand),
}
