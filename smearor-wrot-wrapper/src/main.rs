//! smearor-wrot-wrapper: CLI application providing the complete window solution

pub mod cli;
pub mod config_file;

use crate::cli::args::ApplicationArguments;
use clap::Parser;
use miette::IntoDiagnostic;
use smearor_wrot_application::Application;
use smearor_wrot_application::ApplicationState;
use smearor_wrot_application::init_logging;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), miette::Error> {
    init_logging();

    // Parse command line arguments
    let args = ApplicationArguments::parse();

    // Load configuration file if provided
    let args = args.load_and_merge_config()?;

    // Convert arguments to configuration
    let config: ApplicationState = args.into();

    // Instantiate application with configuration
    let application = Application::new(config).into_diagnostic()?;

    // Run application
    application.run()?;

    Ok(())
}
