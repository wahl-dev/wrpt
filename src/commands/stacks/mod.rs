pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::stacks::args::{StackCommand, StackSubCommand};
use crate::commands::wrpt::GlobalArgs;

pub fn handler(stack: StackCommand, global_args: GlobalArgs) -> Result<(), ()> {
    let command = stack.command;

    match command {
        StackSubCommand::List(command) => handlers::list::handler(command, global_args),
        StackSubCommand::Deploy(command) => handlers::deploy::handler(command, global_args),
        StackSubCommand::Remove(command) => handlers::remove::handler(command, global_args),
        StackSubCommand::ResourceControl(command) => {
            handlers::resource_control::handler(command, global_args)
        }
        StackSubCommand::Stop(command) => handlers::stop::handler(command, global_args),
    }
}
