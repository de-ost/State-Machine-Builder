/*!
This module contains functions to check the validity of the state machine.
*/

use std::collections::HashSet;

use crate::state_machines::Machine;

/// Check if the elements in the state machine are unique.
/// The elements are the states, input alphabet, and output alphabet.
///
/// # Arguments
///
/// * `machine` - The State Machine to check.
///
/// # Returns
///
/// * `Ok(())` if the elements are unique.
/// * `Err(String)` if the elements are not unique. The error message contains the duplicate elements.
pub fn validate_unique_elements<T, U>(machine: &Machine<T, U>) -> Result<(), String> {
    let mut unique_elements: HashSet<String> = HashSet::new();
    let mut duplicates: HashSet<String> = HashSet::new();

    for state in &machine.states {
        if unique_elements.contains(state) {
            duplicates.insert(state.to_string());
        }
        unique_elements.insert(state.to_string());
    }

    for alphabet in &machine.input_alphabet {
        if unique_elements.contains(alphabet) {
            duplicates.insert(alphabet.to_string());
        }
        unique_elements.insert(alphabet.to_string());
    }

    for symbol in &machine.output_alphabet {
        if unique_elements.contains(symbol) {
            duplicates.insert(symbol.to_string());
        }
        unique_elements.insert(symbol.to_string());
    }

    if duplicates.len() > 0 {
        return Err(format!(
            "Duplicate elements found: {}",
            duplicates
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        ));
    }
    Ok(())
}

/// Check if the end states are valid.
/// The end states must be a subset of the states.
/// The end states can be empty.
///
/// # Arguments
///
/// * `machine` - The State Machine to check.
///
/// # Returns
///
/// * `Ok(())` if the end states are valid.
/// * `Err(String)` if the end states are not valid.
pub fn validate_end_states<T, U>(machine: &Machine<T, U>) -> Result<(), String> {
    let states: HashSet<&String> = machine.states.iter().collect();
    let end_states: HashSet<&String> = machine.end_states.iter().collect();

    if !end_states.is_subset(&states) {
        return Err("The end states must be a subset of the states.".to_string());
    }

    Ok(())
}

/// Check if none of the states or symbols starts with a number or are empty.
/// This is to prevent the user from using numbers as states or symbols.
/// The empty string is not allowed as a state or symbol.
/// This is to prevent the user from using the empty string as a state or symbol.
/// C and many other languages do not allow variable names to start with a number.
pub fn validate_legal_variable_name<T, U>(machine: &Machine<T, U>) -> Result<(), String> {
    for state in &machine.states {
        if let Some(first_char) = state.chars().next() {
            if first_char.is_numeric() {
                return Err(format!("The state {} starts with a number.", state));
            }
        } else {
            return Err(format!("The state {} is empty.", state));
        }
    }

    for symbol in &machine.input_alphabet {
        if let Some(first_char) = symbol.chars().next() {
            if first_char.is_numeric() {
                return Err(format!("The input {} starts with a number.", symbol));
            }
        } else {
            return Err(format!("The input {} is empty.", symbol));
        }
    }

    for symbol in &machine.output_alphabet {
        if let Some(first_char) = symbol.chars().next() {
            if first_char.is_numeric() {
                return Err(format!("The output {} starts with a number.", symbol));
            }
        } else {
            return Err(format!("The output {} is empty.", symbol));
        }
    }

    // TODO check if strings do not contain spaces

    Ok(())
}
