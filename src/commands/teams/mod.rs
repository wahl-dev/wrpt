pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::teams::args::{TeamCommand, TeamSubCommand};
use crate::commands::wrpt::GlobalArgs;

pub fn handler(endpoint: TeamCommand, global_args: GlobalArgs) -> Result<(), ()> {
    let command = endpoint.command;

    match command {
        TeamSubCommand::List(command) => handlers::list::handler(command, global_args),
    }
}
