use std::{fs, process};

mod argument_parser;
use crate::argument_parser::Cli;

mod checks;
use crate::checks::check_if_config_elements_unique;

use clap::Parser;

mod state_machines;
use crate::state_machines::moore::MooreMachine;
use crate::state_machines::StateMachine;

mod c_generator;
use crate::c_generator::generate_c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments.
    let cli = Cli::parse();
    let yaml_str = fs::read_to_string(cli.yaml_file)?;

    // Parse the YAML file.
    let moore_machine: MooreMachine = serde_yaml::from_str(&yaml_str).unwrap_or_else(|e| {
        eprintln!("Error parsing YAML: {:?}", e);
        process::exit(1);
    });

    // Run the checks.
    check_if_config_elements_unique(&moore_machine)?;

    let state_machine = StateMachine::Moore(moore_machine);

    // Generate the C code.
    generate_c(&state_machine);

    Ok(())
}
