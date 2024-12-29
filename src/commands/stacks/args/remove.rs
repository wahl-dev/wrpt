use clap::Args;

#[derive(Debug, Args)]
pub struct StackRemoveCommand {
    /// Name of the stack
    pub stack_name: String,
    
    /// Id of the environment (endpoint) that will be used
    #[arg(short = 'E', long)]
    pub endpoint: u32,
}
