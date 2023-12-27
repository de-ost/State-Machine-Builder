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
