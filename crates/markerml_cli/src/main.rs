//! This is a crate that provides CLI
//! for the MarkerML language.
//!
//! For the full grammar overview,
//! refer to the [`markerml`]() crate.
//!
//! The resulting program provides several commands:
//! - Command to convert file with MarkerML code into HTML
//! ```sh
//! markerml_cli convert --input file.txt --output file.html
//! ```
//!
//! - Command to watch the given file with MarkerML code
//!   and track changes on a live-reloading HTML page
//! ```sh
//! markerml_cli watch --input file.txt
//! ```
//!
//! - Command to display credits information
//! ```sh
//! markerml_cli credits
//! ```
//!
//! - Command to display list of commands
//! ```sh
//! markerml_cli help
//! ```
//!

mod args;
mod common;
mod web_server;

use crate::args::{Args, Command};
use anyhow::{Context, Result};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    match Args::read().command {
        Command::Convert { input, output } => convert_file(input, output)?,
        Command::Watch { input, port } => watch_file(input, port).await?,
        Command::Credits => display_credits(),
        Command::Help => display_help(),
    };

    Ok(())
}

fn convert_file(input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    println!("Converting file {}", input.as_ref().display());
    common::check_file_exists(input.as_ref())?;
    let file = common::parse_file(input.as_ref())?;
    println!("Successfully converted");

    std::fs::write(&output, file).with_context(|| {
        format!(
            "Couldn't write output to file {}",
            output.as_ref().display()
        )
    })?;
    println!(
        "Successfully saved output to file {}",
        output.as_ref().display()
    );

    Ok(())
}

async fn watch_file(input: impl AsRef<Path>, port: Option<u16>) -> Result<()> {
    let port = port.unwrap_or(3002);

    println!("Watching file {}...", input.as_ref().display());
    common::check_file_exists(input.as_ref())?;
    println!("Webserver listening at http://localhost:{port}");
    web_server::run_web_server(input.as_ref(), port).await
}

fn display_credits() {
    println!("Made by Ruslan Omelchuk | https://github.com/rchuk");
}

fn display_help() {
    println!("Usage: markerml_cli <command> <options>");
    println!("Commands:");
    println!("  convert --input <input_file> --output <output_file>    Convert specified file");
    println!(
        "  watch --input <input_file>                             Run webserver for specified file"
    );
    println!(
        "  credits                                                Display credits information"
    );
    println!(
        "  help                                                   Display this list of commands"
    );
}
