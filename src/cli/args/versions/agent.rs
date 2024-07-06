use clap::Parser;

/// Dux agent use case : apply a task list (local or remote file) on localhost
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version)]
pub struct CliArgsAgent {
    /// Path to configuration file
    #[arg(short, long)]
    pub conf: Option<String>,

    /// Path to TaskList file
    #[arg(short, long)]
    pub tasklist: Option<String>,

    /// Username to use on localhost
    #[arg(short, long)]
    pub user: Option<String>,

    /// Password to use on localhost
    #[arg(short, long)]
    pub password: Option<String>,
}
