[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


# State Machine Builder

This is a simple state machine builder, a tool designed to construct a state machine and generate C code for integration into any C project. The generated state machine follows the principles of a finite state machine (FSM). To use the builder, the user must provide a YAML file that outlines the state machine's definition, including states, events, and transitions.


##  Usage

This tool can be used like any other command-line tool. The following command will generate a C file from a YAML file:

```bash
./state_machine_builder -y <yaml-file> -o <output-folder> -n <name>
```

A correct YAML file can be found in the `resources` folder. For now only Moore machines are supported.


### Command-line Options

- ` --yaml-file` or `-y`: The YAML file with the state machine.
- ` --output` or `-o`: Name of the output file/directory. If not set, the name of the input file/directory will be used. If the input is a directory, the output will be a directory with the same name. If the input is a file, the output will be a file with the same name.
- ` --name` or `-n`: Name of the state machine. This will be used as the name of the C struct.


### Building the Project

This project is implemented in Rust, and to compile it, you must have Rust and Cargo installed on your system. Follow the steps below to build the project:

#### Install Rust and Cargo:
If you haven't installed Rust and Cargo yet, you can do so by following the official instructions available at [Rustup](https://rustup.rs/).

#### Build the Project:
Once Rust and Cargo are installed, navigate to the project directory in your terminal and execute the following command to build the project:

```bash	
cargo build --release
```

This command compiles the project in release mode, optimizing the executable for performance. After a successful build, you can find the compiled executable in the target/release directory within the project.


### Example of a Moore Machine

| Section             | Explanation                                                            |
|---------------------|------------------------------------------------------------------------|
| States              | List of states in the state machine.                                   |
| Input Alphabet      | List of symbols in the input alphabet.                                 |
| Transitions         | Describes transitions between states based on input symbols.           |
| Output Function     | Specifies the output symbols corresponding to specific states.         |
| Start State         | Indicates the initial state of the state machine.                      |
| End States          | Lists states that are considered end or final states.                  |
| Output Alphabet     | List of symbols in the output alphabet.                                |


```yaml	
states:
  - "q1"
  - "q2"
  - "q3"
input_alphabet:
  - "i0"
  - "i1"
transitions:
  - current_state: "q1"
    read_symbol: ["i0", "i1"]
    new_state: "q2"
  - current_state: "q2"
    read_symbol: ["i1"]
    new_state: "q3"
output_function:
  - current_state: q1
    output_symbol: ["o5", "o6"]
start_state: "q1"
end_states:
  - "q3"
output_alphabet:
  - "o5"
  - "o6"
```
