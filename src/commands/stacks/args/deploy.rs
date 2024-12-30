use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct StackDeployCommand {
    /// Name of the stack
    pub stack_name: String,

    /// Id of the environment (endpoint) that will be used
    #[arg(short = 'E', long)]
    pub endpoint: u32,

    /// Path to docker compose/stack file
    #[arg(short, long)]
    pub compose_file: PathBuf,

    /// Path to a file of environment variables, to be used by the stack
    #[arg(short, long)]
    pub env_file: Option<PathBuf>,

    /// Whether to prune unused containers or not
    #[arg(long)]
    pub prune: bool,

    /// Force a pulling to current image with the original tag though the image is already the latest
    #[arg(long)]
    pub pull_image: bool,
}
