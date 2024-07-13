use clap::Parser;

use crate::cli::args::versions::standard::CliArgsStandard;
use crate::error::Error;

// TODO : for each of these functions, add some arguments checking and default values addition when omitted

pub fn parse_cli_args_standard() -> Result<CliArgsStandard, Error> {
    Ok(CliArgsStandard::parse())
}
