mod argument_parser;
mod c_generator;
mod checks;
mod state_machines;

use argument_parser::Cli;
use clap::Parser;
use state_machines::{MealyMachine, MooreMachine, StateMachine};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let yaml_str = fs::read_to_string(cli.yaml_file)?;
    let name = cli.name;

    let state_machine = parse_yaml(&yaml_str)?;

    match &state_machine {
        StateMachine::Moore(machine) => {
            checks::validate_unique_elements(&machine)?;
            checks::validate_end_states(&machine)?;
        }
        StateMachine::Mealy(machine) => {
            checks::validate_unique_elements(&machine)?;
            checks::validate_end_states(&machine)?;
        }
    };

    c_generator::generate(&name, &state_machine)?;

    Ok(())
}

/// Parse the YAML file and return a `MooreMachine` or a `MealyMachine`.
/// If the YAML file contains both a Moore and a Mealy machine or neither of them, an error is returned.
fn parse_yaml(yaml_str: &str) -> Result<StateMachine, &'static str> {
    let moore_machine: Result<MooreMachine, _> = serde_yaml::from_str(&yaml_str);
    let mealy_machine: Result<MealyMachine, _> = serde_yaml::from_str(&yaml_str);

    match (moore_machine, mealy_machine) {
        (Ok(moore_machine), Err(_)) => Ok(StateMachine::Moore(moore_machine)),
        (Err(_), Ok(mealy_machine)) => Ok(StateMachine::Mealy(mealy_machine)),
        (Err(_), Err(_)) => Err(
            "The YAML file does not contain a Moore or a Mealy machine. Please check the syntax of the file.",
        ),
        (Ok(_), Ok(_)) => Err("The YAML file contains both a Moore and a Mealy machine."),
    }
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
        let state_machine = parse_yaml(&yaml_str).unwrap();

        let val = match state_machine {
            StateMachine::Moore(m) => checks::validate_unique_elements(&m),
            _ => panic!("Wrong state machine type."),
        };

        assert!(val.is_ok());
    }

    #[test]
    fn test_end_states_moore() {
        let yaml_str = include_str!("../resources/test_moore.yaml");
        let state_machine = parse_yaml(&yaml_str).unwrap();

        let val = match state_machine {
            StateMachine::Moore(m) => checks::validate_end_states(&m),
            _ => panic!("Wrong state machine type."),
        };

        assert!(val.is_ok());
    }
}
