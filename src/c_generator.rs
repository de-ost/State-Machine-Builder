/*!
C code generator

This module generates C code from a state machine. The generated code is
based on the state machine's type (Moore or Mealy).

The generated code is written to a file or directory.
*/

use crate::state_machines::{Machine, MooreMachine, StateMachine};
use crate::files::Files;

/// The C files. Contains the header and source file content as strings.
#[derive(Debug)]
struct CFiles {
    header: String, // The header file.
    source: String, // The source file.
}

/// Generates the C code for the state machine. The code is written to a file or directory.
/// The name is used for the file name.
pub fn generate(name: &str, state_machine: &StateMachine, mut files: Files) -> Result<Files, String> {
    let mut c_files = CFiles {
        header: include_str!("../resources/templates/c/header.h").to_string(),
        source: include_str!("../resources/templates/c/source.c").to_string(),
    };

    // Replace the name.
    replace_code(&mut c_files, "NAME", &name);

    match &state_machine {
        StateMachine::Moore(m) => {
            replace_header_code(&m, &mut c_files);
            reset_output(&mut c_files, &m);
            let cases = moore::all_cases(&m);
            replace_code(&mut c_files, "CASE", &cases);
        }
        StateMachine::Mealy(m) => {
            replace_header_code(&m, &mut c_files);
            reset_output(&mut c_files, &m);
            // TODO Implement Mealy machine.
            return Err("Mealy machines are not supported yet.".to_string());
        }
    }

    files.add_file(format!("{}.h", name), c_files.header);
    files.add_file(format!("{}.c", name), c_files.source);

    Ok(files)
}

/// Replaces the code in the C files.
fn replace_code(c_file: &mut CFiles, code: &str, replacement: &str) {
    let code = format!("/*CODE:{}*/", code);
    c_file.header = c_file.header.replace(&code, replacement);
    c_file.source = c_file.source.replace(&code, replacement);
}

/// Replaces the code in the header file. This includes the name, states, inputs, outputs, and initial state.
fn replace_header_code<T, O>(m: &Machine<T, O>, c_files: &mut CFiles) {
    set_declaration(c_files, &m.states, "STATES_ENUM", "", ',');
    set_declaration(
        c_files,
        &m.input_alphabet,
        "INPUTS_DECLARATION",
        "bool",
        ';',
    );
    set_declaration(
        c_files,
        &m.output_alphabet,
        "OUTPUTS_DECLARATION",
        "bool",
        ';',
    );
    replace_code(c_files, "INITIAL_STATE", &m.start_state);
}

/// Sets the declaration of the inputs or outputs. This is used for the header file.
fn set_declaration(
    c_file: &mut CFiles,
    inputs: &[String],
    to_replace: &str,
    data_type: &str,
    separator: char,
) {
    let mut declaration = String::new();

    for input in inputs {
        declaration.push_str(&format!("    {} {}{}\n", data_type, input, separator));
    }

    replace_code(c_file, &to_replace, &declaration);
}

fn reset_output<T, O>(c_file: &mut CFiles, m: &Machine<T, O>) {
    let mut reset = String::new();

    for output in &m.output_alphabet {
        reset.push_str(&format!("    output->{} = false;\n", output));
    }

    replace_code(c_file, "RESET_OUTPUT", &reset);
}

mod moore {
    use super::*;
    use crate::state_machines::MooreTransition;

    /// Generates the code for the states. This is used for the source file.
    pub fn all_cases(m: &MooreMachine) -> String {
        let case = include_str!("../resources/templates/c/case.c");
        let mut code_code = String::new();

        for state in &m.states {
            let mut s = case.replace("/*CODE:CASE_NAME*/", state);

            let case_code = case_code(&m, state);
            s = s.replace("/*CODE:CASE_CODE*/", &case_code);

            code_code.push_str(&s);
            code_code.push('\n');
        }

        code_code
    }

    fn case_code(m: &MooreMachine, state: &str) -> String {
        let mut code = include_str!("../resources/templates/c/moore/case_code.c").to_string();

        // Outputs
        // TODO Check if only one output function fits the current state.
        let outputs = m
            .output_function
            .iter()
            .filter(|o| o.current_state == *state)
            // take only the first output function
            .take(1)
            .map(|o| {
                o.output_symbol
                    .iter()
                    .map(|s| format!("output->{} = true;", s))
                    .collect::<Vec<String>>()
                    .join("\n        ")
            })
            .collect::<Vec<String>>();

        code = code.replace(
            "/*CODE:SET_OUTPUT*/",
            &outputs.first().unwrap_or(&String::new()),
        );

        // Ifs
        let transitions = m
            .transitions
            .iter()
            .filter(|t| t.current_state == *state)
            .collect::<Vec<&MooreTransition>>();

        let mut transitions_code = String::new();
        for transition in transitions {
            let mut if_code = include_str!("../resources/templates/c/moore/case_if.c").to_string();

            // TODO Make false checks possible.
            let conditions = transition
                .read_symbol
                .iter()
                .map(|s| format!("input.{}", s))
                .collect::<Vec<String>>()
                .join(" && ");

            if_code = if_code.replace("/*CODE:IF*/", &conditions);

            let new_state = format!("*state = {};", &transition.new_state);
            if_code = if_code.replace("/*CODE:SET_NEW_STATE*/", &new_state);

            transitions_code.push_str(&if_code);
            transitions_code.push('\n');
        }

        code = code.replace("/*CODE:CASE_IFS*/", &transitions_code);

        code
    }
}
