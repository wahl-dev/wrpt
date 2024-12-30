mod endpoints;
mod stacks;
mod wrpt;
mod helpers;
mod consts;

use clap::{Parser, Subcommand};
use crate::commands::Command::{Endpoint, Stack};
use crate::commands::wrpt::{init_logger, WrptArgs};

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Endpoint subcommands (list, ...)
    Endpoint(endpoints::args::EndpointCommand),

    /// Stacks subcommands (list, deploy, inspect, ...)
    Stack(stacks::args::StackCommand),
}

pub fn init() -> Result<(), ()> {
    let args: WrptArgs = WrptArgs::parse();

    init_logger(&args);
    
    match args.command {
        Stack(command) => stacks::handler(command, args.global_args),
        Endpoint(command) => endpoints::handler(command, args.global_args),
    }
}
