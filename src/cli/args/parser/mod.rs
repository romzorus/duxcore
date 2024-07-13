use clap::Parser;

use crate::cli::args::versions::scalable::CliArgsScalableController;
use crate::cli::args::versions::scalable::CliArgsScalableWorker;
use crate::cli::args::versions::standard::CliArgsStandard;
use crate::error::Error;

// TODO : for each of these functions, add some arguments checking and default values addition when omitted
pub fn parse_cli_args_scalable_worker() -> Result<CliArgsScalableWorker, Error> {
    Ok(CliArgsScalableWorker::parse())
}

pub fn parse_cli_args_scalable_controller() -> Result<CliArgsScalableController, Error> {
    Ok(CliArgsScalableController::parse())
}

pub fn parse_cli_args_standard() -> Result<CliArgsStandard, Error> {
    Ok(CliArgsStandard::parse())
}
