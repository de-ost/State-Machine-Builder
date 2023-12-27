/*!
This module contains functions to check the validity of the state machine.
*/

use std::collections::HashSet;

use crate::state_machines::Machine;

/// Check if the elements in the config are unique.
///
/// # Arguments
///
/// * `config` - The config to check.
///
/// # Returns
///
/// * `Ok(())` if the elements are unique.
/// * `Err(String)` if the elements are not unique.
pub fn check_if_config_elements_unique<T, U>(config: &Machine<T, U>) -> Result<(), String> {
    let mut unique_elements: HashSet<String> = HashSet::new();
    let mut duplicates: HashSet<String> = HashSet::new();

    for state in &config.states {
        if unique_elements.contains(state) {
            duplicates.insert(state.to_string());
        }
        unique_elements.insert(state.to_string());
    }

    for alphabet in &config.input_alphabet {
        if unique_elements.contains(alphabet) {
            duplicates.insert(alphabet.to_string());
        }
        unique_elements.insert(alphabet.to_string());
    }

    for symbol in &config.output_alphabet {
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
