use std::{fs, process};

mod argument_parser;
use crate::argument_parser::Cli;

mod checks;
use crate::checks::check_if_config_elements_unique;

use clap::Parser;
use state_machines::{MealyMachine, StateMachine};

mod state_machines;
use crate::state_machines::MooreMachine;

mod c_generator;
use crate::c_generator::generate_c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments.
    let cli = Cli::parse();
    let yaml_str = fs::read_to_string(cli.yaml_file)?;
    let name = cli.name;

    // Parse the YAML file.
    let state_machine = parse_yaml(&yaml_str);

    // Check if the elements in the config are unique.
    match &state_machine {
        StateMachine::Moore(m) => check_if_config_elements_unique(&m),
        StateMachine::Mealy(m) => check_if_config_elements_unique(&m),
    }?;

    // Generate the C code.
    generate_c(&name, &state_machine)?;

    Ok(())
}

fn parse_yaml(yaml_str: &str) -> StateMachine {
    let moore_machine: Result<MooreMachine, _> = serde_yaml::from_str(&yaml_str);

    let mealy_machine: Result<MealyMachine, _> = serde_yaml::from_str(&yaml_str);

    // Check if the YAML file contains a Moore or a Mealy machine.
    let state_machine = match (moore_machine, mealy_machine) {
        (Ok(moore_machine), Err(_)) => StateMachine::Moore(moore_machine),
        (Err(_), Ok(mealy_machine)) => StateMachine::Mealy(mealy_machine),
        (Err(e1), Err(e2)) => {
            eprintln!("Error parsing YAML (moore): {:?}", e1);
            eprintln!("Error parsing YAML (mealy): {:?}", e2);
            process::exit(1);
        }
        (Ok(_), Ok(_)) => {
            eprintln!("Error parsing YAML: Both Moore and Mealy machines found.");
            process::exit(1);
        }
    };
    state_machine
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_moore_yaml() {
        let yaml_str = include_str!("../resources/test_moore.yaml");
        let moore_machine: Result<MooreMachine, _> = serde_yaml::from_str(&yaml_str);

        assert!(moore_machine.is_ok());
    }

    #[test]
    fn test_parse_not_mealy_yaml() {
        let yaml_str = include_str!("../resources/test_moore.yaml");
        let mealy_machine: Result<MealyMachine, _> = serde_yaml::from_str(&yaml_str);

        assert!(mealy_machine.is_err());
    }

    #[test]
    fn test_config_moore() {
        let yaml_str = include_str!("../resources/test_moore.yaml");
        let state_machine = parse_yaml(&yaml_str);

        let val = match state_machine {
            StateMachine::Moore(m) => check_if_config_elements_unique(&m),
            _ => panic!("Wrong state machine type."),
        };

        assert!(val.is_ok());
    }
}
