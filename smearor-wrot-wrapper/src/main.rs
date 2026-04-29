//! smearor-wrot-wrapper: CLI application providing the complete window solution

pub mod args;

use std::env;
use std::sync::Arc;
use clap::Parser;
use miette::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::args::Arguments;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Force the OpenGL renderer for GSK to ensure better compatibility with textures.
    unsafe { env::set_var("GSK_RENDERER", "gl"); }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let command_line_arguments = Arc::new(Arguments::parse());

    info!("Starting smearor-wrot");
    info!("CLI application initialized");

    Ok(())
}
