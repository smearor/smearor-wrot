//! smearor-wrot-wrapper: CLI application providing the complete window solution

pub mod cli;
pub mod config_file;

use crate::cli::args::SmearorWindowRotationArguments;
use clap::Parser;
use smearor_wrot_application::CompositorApplication;
use smearor_wrot_application::init_logging;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Note: GSK_RENDERER is not set here to allow users to set it via environment variable
    // If GSK_RENDERER is set in the parent process, it will be inherited by child processes

    init_logging();

    // Parse command line arguments
    let args = SmearorWindowRotationArguments::parse();

    // Load configuration file if provided
    let args = args.load_and_merge_config()?;

    let application = CompositorApplication::builder().config(args.into()).build();
    application.run()?;
    Ok(())
}
