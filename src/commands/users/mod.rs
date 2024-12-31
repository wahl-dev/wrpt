pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::users::args::{UserCommand, UserSubCommand};
use crate::commands::wrpt::GlobalArgs;

pub fn handler(endpoint: UserCommand, global_args: GlobalArgs) -> Result<(), ()> {
    let command = endpoint.command;

    match command {
        UserSubCommand::List(command) => handlers::list::handler(command, global_args),
    }
}
