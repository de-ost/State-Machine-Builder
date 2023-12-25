/*!
This module contains the definition of a finite state machine.
It is used to represent both Moore and Mealy machines.
The difference between the two is that Moore machines have an output function that depends only on the current state,
while Mealy machines have an output function that depends on the current state and the input symbol.
The definition of the two types of machines can be found in the `moore` and `mealy` modules.
*/

/// Represents a finite state machine. It can be either a Moore or a Mealy machine.
#[derive(Debug)]
pub enum StateMachine {
    Moore(moore::MooreMachine),
    _Mealy(mealy::MealyMachine),
}

/// This module contains the implementation of a Moore machine.
pub mod moore {
    use serde::Deserialize;

    /// Represents a Moore machine.
    #[derive(Debug, Deserialize)]
    pub struct MooreMachine {
        pub states: Vec<String>,                       // Q
        pub input_alphabet: Vec<String>,               // Σ
        pub output_alphabet: Vec<String>,              // Ω
        pub transitions: Vec<MooreTransition>,         // δ
        pub output_function: Vec<MooreOutputFunction>, // λ
        pub start_state: String,                       // q0
        pub end_states: Vec<String>,                   // F
    }

    /// Represents a transition in a Moore machine.
    #[derive(Debug, Deserialize)]
    pub struct MooreTransition {
        pub current_state: String,    // q
        pub read_symbol: Vec<String>, // s
        pub new_state: String,        // q'
    }

    /// Represents an output function in a Moore machine.
    #[derive(Debug, Deserialize)]
    pub struct MooreOutputFunction {
        pub current_state: String, // q
        pub output_symbol: String, // o
    }
}

/// This module contains the definition of a Mealy machine.
pub mod mealy {
    use serde::Deserialize;

    /// Represents a Mealy machine.
    #[derive(Debug, Deserialize)]
    pub struct MealyMachine {
        pub states: Vec<String>,               // Q
        pub input_alphabet: Vec<String>,       // Σ
        pub output_alphabet: Vec<String>,      // Ω
        pub transitions: Vec<MealyTransition>, // δ
        pub start_state: String,               // q0
        pub end_states: Vec<String>,           // F
    }

    /// Represents a transition in a Mealy machine.
    #[derive(Debug, Deserialize)]
    pub struct MealyTransition {
        pub current_state: String,    // q
        pub read_symbol: Vec<String>, // s
        pub new_state: String,        // q'
        pub output_symbol: String,    // o
    }
}
