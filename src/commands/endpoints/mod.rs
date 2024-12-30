pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::endpoints::args::{EndpointCommand, EndpointSubCommand};
use crate::commands::wrpt::GlobalArgs;

pub fn handler(endpoint: EndpointCommand, global_args: GlobalArgs) -> Result<(), ()> {
    let command = endpoint.command;

    match command {
        EndpointSubCommand::List(command) => handlers::list::handler(command, global_args),
    }
}
