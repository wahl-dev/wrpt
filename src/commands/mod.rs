mod consts;
mod endpoints;
mod helpers;
mod stacks;
mod teams;
mod users;
mod wrpt;

use crate::commands::wrpt::{init_logger, WrptArgs};
use crate::commands::Command::{Endpoint, Stack, Team, User};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Endpoint subcommands (list, ...)
    Endpoint(endpoints::args::EndpointCommand),

    /// Stacks subcommands (list, deploy, inspect, ...)
    Stack(stacks::args::StackCommand),

    /// Teams subcommands (list, ...)
    Team(teams::args::TeamCommand),

    /// Users subcommands (list, ...)
    User(users::args::UserCommand),
}

pub fn init() -> Result<(), ()> {
    let args: WrptArgs = WrptArgs::parse();

    init_logger(&args);

    match args.command {
        Endpoint(command) => endpoints::handler(command, args.global_args),
        Stack(command) => stacks::handler(command, args.global_args),
        Team(command) => teams::handler(command, args.global_args),
        User(command) => users::handler(command, args.global_args),
    }
}
