/*!
This module contains the argument parser for the CLI.
*/

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "State Machine Builder", author = "Dennis Ostermann", version, about, long_about = None)]
pub struct Cli {
    /// The YAML file with the state machine.
    #[arg(short, long, value_name = "FILE")]
    pub yaml_file: PathBuf,

    /// Name of the output file/directory.
    /// If not set, the name of the input file/directory will be used.
    /// If the input is a directory, the output will be a directory with the same name.
    /// If the input is a file, the output will be a file with the same name.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Name of the state machine. This will be used as the name of the C struct.
    #[arg(short, long, value_name = "NAME")]
    pub name: String,
}
