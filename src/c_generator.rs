/*!
C code generator

This module generates C code from a state machine. The generated code is
based on the state machine's type (Moore or Mealy).

The generated code is written to a file or directory.
*/

use crate::state_machines::{Machine, StateMachine, MooreMachine};

/// The C files. Contains the header and source file content as strings.
#[derive(Debug)]
struct CFiles {
    header: String, // The header file.
    source: String, // The source file.
}

/// Generates the C code for the state machine. The code is written to a file or directory.
/// The name is used for the file name.
pub fn generate(name: &str, state_machine: &StateMachine) -> Result<(), String> {
    let mut c_files = CFiles {
        header: include_str!("../resources/templates/c/header.h").to_string(),
        source: include_str!("../resources/templates/c/source.c").to_string(),
    };

    // Replace the name.
    replace_code(&mut c_files, "NAME", &name);

    match &state_machine {
        StateMachine::Moore(m) => {
            // Header
            replace_header_code(&m, &mut c_files);

            // Source
        }
        StateMachine::Mealy(m) => {
            // Header
            replace_header_code(&m, &mut c_files);

            // Source
        }
    }

    Ok(())
}

/// Replaces the code in the C files.
fn replace_code(c_file: &mut CFiles, code: &str, replacement: &str) {
    let code = format!("/*CODE:{}*/", code);
    c_file.header = c_file.header.replace(&code, replacement);
    c_file.source = c_file.source.replace(&code, replacement);
}

/// Replaces the code in the header file. This includes the name, states, inputs, outputs, and initial state.
fn replace_header_code<T, O>(m: &Machine<T, O>, c_files: &mut CFiles) {
    set_declaration(c_files, &m.states, "STATES_ENUM", "");
    set_declaration(c_files, &m.input_alphabet, "INPUTS_DECLARATION", "bool");
    set_declaration(c_files, &m.output_alphabet, "OUTPUTS_DECLARATION", "bool");
    replace_code(c_files, "INITIAL_STATE", &m.start_state);
}

/// Sets the declaration of the inputs or outputs. This is used for the header file.
fn set_declaration(c_file: &mut CFiles, inputs: &[String], to_replace: &str, data_type: &str) {
    let mut declaration = String::new();

    for input in inputs {
        declaration.push_str(&format!("    {} {};\n", data_type, input));
    }

    replace_code(c_file, &to_replace, &declaration);
}

/// Generates the code for the states. This is used for the source file.
fn moore_case_code(m: &MooreMachine) -> String {
    let case = include_str!("../resources/templates/c/case.c");
    let code = String::new();

    for state in &m.states {
        let mut s = case.replace("/*CODE:STATE*/", state);
        todo!("finish this"); // TODO finish this
    }

    code
}

/*
/*CODE:NAME*/
/*CODE:STATES_ENUM*/
/*CODE:INPUTS_DECLARATION*/
/*CODE:OUTPUTS_DECLARATION*/
/*CODE:INITIAL_STATE*/
*/
