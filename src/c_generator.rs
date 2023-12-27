/*!
C code generator

This module generates C code from a state machine. The generated code is
based on the state machine's type (Moore or Mealy).

The generated code is written to a file or directory.
*/

use crate::state_machines::{Machine, StateMachine, MooreMachine};

#[derive(Debug)]
struct CFiles {
    header: String, // The header file.
    source: String, // The source file.
}

pub fn generate_c(name: &str, state_machine: &StateMachine) -> Result<(), String> {
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

fn replace_code(c_file: &mut CFiles, code: &str, replacement: &str) {
    let code = format!("/*CODE:{}*/", code);
    c_file.header = c_file.header.replace(&code, replacement);
    c_file.source = c_file.source.replace(&code, replacement);
}

fn replace_header_code<T, O>(m: &Machine<T, O>, c_files: &mut CFiles) {
    set_declaration(c_files, &m.states, "STATES_ENUM", "");
    set_declaration(c_files, &m.input_alphabet, "INPUTS_DECLARATION", "bool");
    set_declaration(c_files, &m.output_alphabet, "OUTPUTS_DECLARATION", "bool");
    replace_code(c_files, "INITIAL_STATE", &m.start_state);
}

fn set_declaration(c_file: &mut CFiles, inputs: &[String], to_replace: &str, data_type: &str) {
    let mut declaration = String::new();

    for input in inputs {
        declaration.push_str(&format!("    {} {};\n", data_type, input));
    }

    replace_code(c_file, &to_replace, &declaration);
}

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
