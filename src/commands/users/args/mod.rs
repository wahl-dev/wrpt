pub(crate) mod list;

use crate::commands::users::args::list::UserListCommand;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    /// List users
    List(UserListCommand),
}
