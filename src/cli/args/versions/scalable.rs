use clap::Parser;


/// Dux scalable use case (worker) : get an assignment from a controller and applies it on a remote host
#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArgsScalableWorker {
    /// Path to configuration file
    #[arg(short, long)]
    pub conf: Option<String>,

    /// Number of threads to use (default = number of CPU of the local machine)
    #[arg(long)]
    pub threads: Option<usize>,
}
