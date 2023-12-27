use serde::Deserialize;

pub type MooreMachine = Machine<MooreTransition, Vec<MooreOutputFunction>>;
pub type MealyMachine = Machine<MealyTransition, Option<()>>;

/// Represents a finite state machine. It can be either a Moore or a Mealy machine.
#[derive(Debug)]
pub enum StateMachine {
    Moore(MooreMachine),
    Mealy(MealyMachine),
}

/// Represents a generic state machine.
#[derive(Debug, Deserialize)]
pub struct Machine<T, M> {
    pub states: Vec<String>,          // Q
    pub input_alphabet: Vec<String>,  // Σ
    pub output_alphabet: Vec<String>, // Ω
    pub transitions: Vec<T>,          // δ
    pub output_function: M,           // λ (moore) or None (mealy machine)
    pub start_state: String,          // q0
    pub end_states: Vec<String>,      // F
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

/// Represents a transition in a Mealy machine.
#[derive(Debug, Deserialize)]
pub struct MealyTransition {
    pub current_state: String,    // q
    pub read_symbol: Vec<String>, // s
    pub new_state: String,        // q'
    pub output_symbol: String,    // o
}
