use clap::{Parser, Subcommand};

/// Command line arguments that the program might receive
#[derive(Parser)]
#[command(name = "markermlcli")]
#[command(about = "CLI for parsing MarkerML into HTML", long_about = None)]
#[command(disable_help_subcommand = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

impl Args {
    pub fn read() -> Self {
        Args::parse()
    }
}

/// Commands that program might perform
#[derive(Subcommand)]
pub enum Command {
    /// Command for converting file with code to html file
    #[clap(about = "Convert specified file")]
    Convert {
        #[arg(short, long, value_name = "Input file")]
        input: String,
        #[arg(short, long, value_name = "Output file")]
        output: String,
    },
    /// Command to start web server and watch for changes in code file
    #[clap(about = "Run webserver for specified file")]
    Watch {
        #[arg(short, long, value_name = "Input file")]
        input: String,
        #[arg(short, long, value_name = "Port")]
        port: Option<u16>,
    },
    /// Command to display credits
    #[clap(about = "Display credits information")]
    Credits,
    /// Command to display list of commands
    #[clap(about = "Display list of commands")]
    Help,
}
